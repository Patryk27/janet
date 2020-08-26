use lib_interface::{Command, Event};
use std::fmt;
use tokio::sync::{mpsc, oneshot};

pub type CommandTx = mpsc::UnboundedSender<Packet<Command>>;
pub type CommandRx = mpsc::UnboundedReceiver<Packet<Command>>;

pub type EventTx = mpsc::UnboundedSender<Packet<Event>>;
pub type EventRx = mpsc::UnboundedReceiver<Packet<Event>>;

pub struct Packet<T> {
    pub item: T,
    pub on_handled: oneshot::Sender<()>,
}

impl<T: fmt::Debug> fmt::Debug for Packet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.item)
    }
}
