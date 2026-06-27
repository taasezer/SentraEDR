use tokio::sync::mpsc;
use crate::models::CommandMessage;

pub struct CommandBus<T: CommandMessage> {
    sender: mpsc::Sender<T>,
}

impl<T: CommandMessage> CommandBus<T> {
    pub fn new(capacity: usize) -> (Self, mpsc::Receiver<T>) {
        let (tx, rx) = mpsc::channel(capacity);
        (Self { sender: tx }, rx)
    }

    /// Reliable delivery with explicit backpressure.
    /// Returns TrySendError if the bounded queue is full.
    pub fn send(&self, command: T) -> Result<(), mpsc::error::TrySendError<T>> {
        self.sender.try_send(command)
    }
}
