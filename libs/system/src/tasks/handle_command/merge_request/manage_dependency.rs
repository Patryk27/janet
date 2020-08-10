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
            .merge_request_dependencies()
            .find_by_src(
                self.user_id,
                self.ctxt.discussion.as_ref(),
                self.merge_request_id,
            )
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
                .merge_request_dependencies()
                .add(&db::NewMergeRequestDependency {
                    user_id: self.user_id,
                    discussion_ext_id: self.ctxt.discussion.as_ref().into(),
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
                .merge_request_dependencies()
                .remove(dependency.id)
                .await?;
        }

        Ok(":+1:".into())
    }
}

#[cfg(test)]
#[cfg(feature = "e2e")]
mod tests {
    use lib_e2e::*;

    mod when_user_adds_comment {
        use super::*;

        mod that_refers_to_existing_merge_request {
            use super::*;

            #[tokio::test(threaded_scheduler)]
            async fn responds_with_acknowledgement() {
                test(async move |ctxt| {
                    ctxt.gitlab.expect_user(1, &gl_mock::user_250()).await;
                    ctxt.gitlab.expect_project(2, &gl_mock::project_10()).await;

                    ctxt.gitlab
                        .expect_merge_request(1, &gl_mock::merge_request_100())
                        .await;

                    ctxt.gitlab
                        .expect_merge_request(2, &gl_mock::merge_request_101())
                        .await;

                    ctxt.gitlab
                        .expect_merge_request_note_created(
                            gl::ProjectId::new(10),
                            gl::MergeRequestIid::new(1),
                            &gl::DiscussionId::new("cafebabe"),
                            "@someone :+1:",
                        )
                        .await;

                    ctxt.janet
                        .spoof_gitlab_webhook(&json!({
                            "event_type": "note",
                            "project": {
                                "id": 10,
                                "namespace": "alpha",
                            },
                            "merge_request": {
                                "id": 100,
                                "iid": 1,
                            },
                            "object_attributes": {
                                "author_id": 250,
                                "description": "@janet depends on !2",
                                "discussion_id": "cafebabe",
                            },
                        }))
                        .await;
                })
                .await;
            }
        }

        mod that_refers_to_missing_merge_request {
            use super::*;

            #[tokio::test(threaded_scheduler)]
            async fn responds_with_error() {
                test(async move |ctxt| {
                    ctxt.gitlab.expect_user(2, &gl_mock::user_250()).await;
                    ctxt.gitlab.expect_project(1, &gl_mock::project_10()).await;

                    ctxt.gitlab
                        .expect_merge_request(1, &gl_mock::merge_request_100())
                        .await;

                    ctxt.gitlab
                        .expect_merge_request_note_created(
                            gl::ProjectId::new(10),
                            gl::MergeRequestIid::new(1),
                            &gl::DiscussionId::new("cafebabe"),
                            "@someone sorry, I couldn't find this merge request - could you please ensure it exists and re-create your comment?",
                        )
                        .await;

                    ctxt.janet
                        .spoof_gitlab_webhook(&json!({
                            "event_type": "note",
                            "project": {
                                "id": 10,
                                "namespace": "alpha",
                            },
                            "merge_request": {
                                "id": 100,
                                "iid": 1,
                            },
                            "object_attributes": {
                                "author_id": 250,
                                "description": "@janet depends on !33",
                                "discussion_id": "cafebabe",
                            },
                        }))
                        .await;
                }).await;
            }
        }
    }
}
