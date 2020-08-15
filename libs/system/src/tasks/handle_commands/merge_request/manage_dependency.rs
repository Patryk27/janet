use super::{HandlerError, HandlerResult};
use crate::prelude::*;

/// Handles the `depends on` & `-depends on` commands
pub async fn handle(
    world: &World,
    ctxt: &int::MergeRequestCommandContext,
    action: int::CommandAction,
    dependency: int::MergeRequestPtr,
) -> HandlerResult<()> {
    let (gl_user, user_id) = sync_user(world, ctxt.user).await?;

    let (gl_project, gl_merge_request, merge_request_id) =
        sync_merge_request_ptr(world, &ctxt.merge_request, &Default::default()).await?;

    let src_context = int::PtrContext {
        namespace_id: Some(gl_project.namespace.id),
        project_id: Some(gl_project.id),
    };

    let (gl_dst_project_id, gl_dst_merge_request_iid) = dependency
        .resolve(&world.gitlab, &src_context)
        .await
        .map_err(|_| HandlerError::MergeRequestNotFound)?;

    Handler {
        world,
        ctxt,
        user_id,
        merge_request_id,
        gl_dst_project_id,
        gl_dst_merge_request_iid,
    }
    .run(action)
    .await?;

    // TODO maybe we could thumbs-up the post instead of sending a comment?

    world
        .gitlab
        .create_merge_request_note(
            gl_project.id,
            gl_merge_request.iid,
            &ctxt.discussion,
            format!("@{} :+1:", gl_user.username),
        )
        .await?;

    Ok(())
}

struct Handler<'a> {
    world: &'a World,
    ctxt: &'a int::MergeRequestCommandContext,
    user_id: db::Id<db::User>,
    merge_request_id: db::Id<db::MergeRequest>,
    gl_dst_project_id: gl::ProjectId,
    gl_dst_merge_request_iid: gl::MergeRequestIid,
}

impl<'a> Handler<'a> {
    async fn run(self, action: int::CommandAction) -> HandlerResult<()> {
        // Since It's totally fine for a merge request pointer to be both resolved _and_
        // invalid - e.g. when user writes `project!123` (assuming the project itself
        // exists) - we have to explicitly check whether the merge request user is
        // talking about exists or not
        if self
            .world
            .gitlab
            .merge_request(self.gl_dst_project_id, self.gl_dst_merge_request_iid)
            .await
            .is_err()
        {
            return Err(HandlerError::MergeRequestNotFound);
        }

        let dependency = self
            .world
            .db
            .get_opt(db::FindMergeRequestDependencies {
                user_id: Some(self.user_id),
                ext_discussion_id: Some(&self.ctxt.discussion),
                src_merge_request_id: Some(self.merge_request_id),
                ..Default::default()
            })
            .await?;

        let (_, _, dst_merge_request_id) = sync_merge_request(
            self.world,
            self.gl_dst_project_id,
            self.gl_dst_merge_request_iid,
        )
        .await?;

        if action.is_add() {
            self.run_add(dependency, dst_merge_request_id).await
        } else {
            self.run_remove(dependency).await
        }
    }

    /// Handles the `depends on` command
    async fn run_add(
        &self,
        dependency: Option<db::MergeRequestDependency>,
        dst_merge_request_id: db::Id<db::MergeRequest>,
    ) -> HandlerResult<()> {
        // It might happen that we already know about this dependency - say, when
        // someone adds the same `depends on !123` comment twice.
        //
        // In order to make the UI less confusing, when that happens, we're just
        // silently ignoring the second request.
        if dependency.is_none() {
            self.world
                .db
                .execute(db::CreateMergeRequestDependency {
                    user_id: self.user_id,
                    ext_discussion_id: self.ctxt.discussion.clone(),
                    src_merge_request_id: self.merge_request_id,
                    dst_merge_request_id,
                })
                .await?;
        }

        Ok(())
    }

    /// Handles the `-depends on` command
    async fn run_remove(
        &self,
        dependency: Option<db::MergeRequestDependency>,
    ) -> HandlerResult<()> {
        // It might happen that we've already removed this dependency - say, when
        // someone adds the same `-depends on !123` comment twice.
        //
        // In order to make the UI less confusing, when that happens, we're just
        // silently ignoring the second request.
        if let Some(dependency) = dependency {
            self.world
                .db
                .execute(db::DeleteMergeRequestDependency { id: dependency.id })
                .await?;
        }

        Ok(())
    }
}
