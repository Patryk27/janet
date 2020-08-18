use super::handle_event;
use crate::{EventRx, SystemDeps};
use anyhow::*;
use std::sync::Arc;
use tokio::stream::StreamExt;

pub async fn handle_events(deps: Arc<SystemDeps>, mut evts: EventRx) -> Result<()> {
    while let Some(evt) = evts.next().await {
        tokio::spawn(handle_event(deps.clone(), evt));
    }

    bail!("Lost connection to the `events` stream")
}
