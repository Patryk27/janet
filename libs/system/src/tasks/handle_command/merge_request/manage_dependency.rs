use super::{HandlerError, HandlerResult};
use crate::utils::{sync_merge_request, sync_merge_request_ptr, sync_user};
use crate::SystemDeps;
use lib_database as db;
use lib_gitlab as gl;
use lib_interface::{CommandAction, MergeRequestCommandContext, MergeRequestPtr, PtrContext};

/// Handles the `depends on` & `-depends on` commands.
pub async fn handle(
    deps: &SystemDeps,
    ctxt: &MergeRequestCommandContext,
    action: CommandAction,
    dependency: MergeRequestPtr,
) -> HandlerResult<()> {
    let (gl_user, user_id) = sync_user(&deps.db, &deps.gitlab, ctxt.user).await?;

    let (gl_project, gl_merge_request, merge_request_id) = sync_merge_request_ptr(
        &deps.db,
        &deps.gitlab,
        &ctxt.merge_request,
        &Default::default(),
    )
    .await?;

    let src_context = PtrContext {
        namespace_id: Some(gl_project.namespace.id),
        project_id: Some(gl_project.id),
    };

    let (gl_dst_project_id, gl_dst_merge_request_iid) = dependency
        .resolve(&deps.gitlab, &src_context)
        .await
        .map_err(|_| HandlerError::MergeRequestNotFound)?;

    Handler {
        deps,
        ctxt,
        gl_user,
        user_id,
        gl_project,
        gl_merge_request,
        merge_request_id,
        gl_dst_project_id,
        gl_dst_merge_request_iid,
    }
    .run(action)
    .await
}

struct Handler<'a> {
    deps: &'a SystemDeps,
    ctxt: &'a MergeRequestCommandContext,

    gl_user: gl::User,
    user_id: db::Id<db::User>,

    gl_project: gl::Project,
    gl_merge_request: gl::MergeRequest,
    merge_request_id: db::Id<db::MergeRequest>,

    gl_dst_project_id: gl::ProjectId,
    gl_dst_merge_request_iid: gl::MergeRequestIid,
}

impl<'a> Handler<'a> {
    async fn run(self, action: CommandAction) -> HandlerResult<()> {
        let response = self.try_run(action).await?;

        self.deps
            .gitlab
            .create_merge_request_note(
                self.gl_project.id,
                self.gl_merge_request.iid,
                &self.ctxt.discussion,
                format!("@{} {}", self.gl_user.username, response),
            )
            .await?;

        Ok(())
    }

    async fn try_run(&self, action: CommandAction) -> HandlerResult<String> {
        // Since It's totally fine for a merge request pointer to be both resolved _and_
        // invalid - e.g. when user writes `project!123` (assuming the project itself
        // exists) - we have to explicitly check whether the merge request user is
        // talking about exists or not
        if self
            .deps
            .gitlab
            .merge_request(self.gl_dst_project_id, self.gl_dst_merge_request_iid)
            .await
            .is_err()
        {
            return Err(HandlerError::MergeRequestNotFound);
        }

        let dependency = self
            .deps
            .db
            .maybe_find_one(db::FindMergeRequestDependencies {
                user_id: Some(self.user_id),
                ext_discussion_id: Some(&self.ctxt.discussion),
                src_merge_request_id: Some(self.merge_request_id),
                ..Default::default()
            })
            .await?;

        let (_, _, dst_merge_request_id) = sync_merge_request(
            &self.deps.db,
            &self.deps.gitlab,
            self.gl_dst_project_id,
            self.gl_dst_merge_request_iid,
        )
        .await?;

        if action.is_add() {
            self.try_run_add(dependency, dst_merge_request_id).await
        } else {
            self.try_run_remove(dependency).await
        }
    }

    /// Handles the `depends on` command.
    async fn try_run_add(
        &self,
        dependency: Option<db::MergeRequestDependency>,
        dst_merge_request_id: db::Id<db::MergeRequest>,
    ) -> HandlerResult<String> {
        // It might happen that we already know about this dependency - say, when
        // someone adds the same `depends on !123` comment twice.
        //
        // In order to make the UI less confusing, when that happens, we're just
        // silently ignoring the second request.
        if dependency.is_none() {
            self.deps
                .db
                .execute(db::CreateMergeRequestDependency {
                    user_id: self.user_id,
                    ext_discussion_id: self.ctxt.discussion.clone(),
                    src_merge_request_id: self.merge_request_id,
                    dst_merge_request_id,
                })
                .await?;
        }

        Ok(":+1:".into())
    }

    /// Handles the `-depends on` command.
    async fn try_run_remove(
        &self,
        dependency: Option<db::MergeRequestDependency>,
    ) -> HandlerResult<String> {
        // It might happen that we've already removed this dependency - say, when
        // someone adds the same `-depends on !123` comment twice.
        //
        // In order to make the UI less confusing, when that happens, we're just
        // silently ignoring the second request.
        if let Some(dependency) = dependency {
            self.deps
                .db
                .execute(db::DeleteMergeRequestDependency { id: dependency.id })
                .await?;
        }

        Ok(":+1:".into())
    }
}
