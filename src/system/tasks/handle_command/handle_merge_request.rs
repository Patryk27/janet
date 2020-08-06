use self::{handle_hi::*, handle_manage_dependency::*};

mod handle_hi;
mod handle_manage_dependency;

use crate::interface::{MergeRequestCommand, MergeRequestCommandContext};
use crate::system::task::TaskContext;
use anyhow::*;
use std::sync::Arc;

pub async fn handle_merge_request(
    tctxt: Arc<TaskContext>,
    ctxt: MergeRequestCommandContext,
    cmd: MergeRequestCommand,
) -> Result<()> {
    let merge_request = ctxt
        .merge_request
        .resolve(&tctxt.gitlab, &Default::default())
        .await?;

    let result = match cmd {
        MergeRequestCommand::Hi => handle_hi(tctxt, ctxt).await,

        MergeRequestCommand::ManageDependency { action, dependency } => {
            handle_manage_dependency(tctxt, ctxt, action, dependency).await
        }
    };

    if let Err(err) = result {
        // TODO notify the user
    }

    Ok(())
}
