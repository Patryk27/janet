use crate::prelude::*;

/// Notifies reminder's user that given reminder has expired and then removes
/// reminder from the database.
#[tracing::instrument(skip(world))]
pub async fn close_reminder(world: &World, reminder: db::Reminder) -> Result<()> {
    tracing::info!("Closing reminder");

    let merge_request = world
        .db
        .get_one(db::FindMergeRequests::id(reminder.merge_request_id))
        .await?;

    let project = world
        .db
        .get_one(db::FindProjects::id(merge_request.project_id))
        .await?;

    let user = world
        .db
        .get_one(db::FindUsers::id(reminder.user_id))
        .await?;

    let gl_user = world.gitlab.user(user.ext_id()).await?;

    let note = reminder
        .message
        .as_ref()
        .map(|msg| format!("reminding: {}", msg))
        .unwrap_or_else(|| "ping, ping!".to_string());

    let note = format!("@{} {}", gl_user.username, note);

    world
        .gitlab
        .create_merge_request_note(
            project.ext_id(),
            merge_request.ext_iid(),
            &reminder.ext_discussion_id(),
            note,
        )
        .await?;

    world
        .db
        .execute(db::DeleteReminder { id: reminder.id })
        .await?;

    Ok(())
}
