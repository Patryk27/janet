use lib_interface::{Command, Event};
use tokio::sync::mpsc;

pub type CommandTx = mpsc::UnboundedSender<CommandPacket>;
pub type CommandRx = mpsc::UnboundedReceiver<CommandPacket>;

pub type EventTx = mpsc::UnboundedSender<EventPacket>;
pub type EventRx = mpsc::UnboundedReceiver<EventPacket>;

#[derive(Clone, Debug)]
pub struct CommandPacket {
    pub command: Command,
    pub responder: Option<mpsc::UnboundedSender<()>>,
}

#[derive(Clone, Debug)]
pub struct EventPacket {
    pub command: Event,
    pub responder: Option<mpsc::UnboundedSender<()>>,
}
