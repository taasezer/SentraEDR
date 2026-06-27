use tokio::sync::broadcast;
use crate::models::EventMessage;
use std::sync::Arc;

pub struct EventBus<T: EventMessage> {
    sender: broadcast::Sender<Arc<T>>,
}

impl<T: EventMessage + 'static> EventBus<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        Self { sender: tx }
    }

    /// Best-effort delivery. Drops event if no active subscribers.
    /// Capacity bounding is handled implicitly by `broadcast::channel`.
    /// Senders never block.
    pub fn publish(&self, event: T) -> Result<usize, broadcast::error::SendError<Arc<T>>> {
        self.sender.send(Arc::new(event))
    }

    /// Strongly typed subscription. Returns a dedicated receiver channel.
    /// Ensures slow subscribers only lag themselves, not others.
    pub fn subscribe(&self) -> broadcast::Receiver<Arc<T>> {
        self.sender.subscribe()
    }
}
