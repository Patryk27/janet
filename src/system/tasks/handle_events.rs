use super::handle_event;
use crate::interface::EventRx;
use crate::system::task::TaskContext;
use anyhow::{bail, Result};
use std::sync::Arc;
use tokio::stream::StreamExt;

pub async fn handle_events(ctxt: Arc<TaskContext>, mut evts: EventRx) -> Result<()> {
    while let Some(evt) = evts.next().await {
        tokio::spawn(handle_event(ctxt.clone(), evt));
    }

    bail!("Lost connection to the `events` stream")
}
