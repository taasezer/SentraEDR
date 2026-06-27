use crate::error::IpcError;
use shared_models::QueueHealth;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tokio::sync::mpsc;

#[derive(Debug)]
struct QueueMetrics {
    name: String,
    capacity: usize,
    depth: AtomicUsize,
    dropped_events: AtomicU64,
}

impl QueueMetrics {
    fn new(name: impl Into<String>, capacity: usize) -> Self {
        Self {
            name: name.into(),
            capacity,
            depth: AtomicUsize::new(0),
            dropped_events: AtomicU64::new(0),
        }
    }

    fn snapshot(&self) -> QueueSnapshot {
        QueueSnapshot {
            name: self.name.clone(),
            capacity: self.capacity,
            depth: self.depth.load(Ordering::Relaxed),
            dropped_events: self.dropped_events.load(Ordering::Relaxed),
        }
    }

    fn health(&self) -> QueueHealth {
        let snapshot = self.snapshot();
        QueueHealth::new(
            snapshot.name,
            snapshot.capacity,
            snapshot.depth,
            snapshot.dropped_events,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueSnapshot {
    pub name: String,
    pub capacity: usize,
    pub depth: usize,
    pub dropped_events: u64,
}

#[derive(Debug)]
pub struct BoundedSender<T> {
    sender: mpsc::Sender<T>,
    metrics: Arc<QueueMetrics>,
}

impl<T> Clone for BoundedSender<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

impl<T> BoundedSender<T> {
    pub fn try_send(&self, value: T) -> Result<(), IpcError> {
        self.sender.try_send(value).map_err(|error| match error {
            mpsc::error::TrySendError::Full(_) => {
                self.metrics.dropped_events.fetch_add(1, Ordering::Relaxed);
                IpcError::QueueFull {
                    queue: self.metrics.name.clone(),
                    capacity: self.metrics.capacity,
                }
            }
            mpsc::error::TrySendError::Closed(_) => IpcError::ReceiverClosed {
                queue: self.metrics.name.clone(),
            },
        })?;
        self.metrics.depth.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn snapshot(&self) -> QueueSnapshot {
        self.metrics.snapshot()
    }

    pub fn health(&self) -> QueueHealth {
        self.metrics.health()
    }
}

#[derive(Debug)]
pub struct BoundedReceiver<T> {
    receiver: mpsc::Receiver<T>,
    metrics: Arc<QueueMetrics>,
}

impl<T> BoundedReceiver<T> {
    pub async fn recv(&mut self) -> Option<T> {
        let value = self.receiver.recv().await;
        if value.is_some() {
            self.metrics.depth.fetch_sub(1, Ordering::Relaxed);
        }
        value
    }

    pub fn snapshot(&self) -> QueueSnapshot {
        self.metrics.snapshot()
    }
}

pub fn bounded_channel<T>(
    name: impl Into<String>,
    capacity: usize,
) -> (BoundedSender<T>, BoundedReceiver<T>) {
    assert!(
        capacity > 0,
        "bounded channel capacity must be greater than zero"
    );
    let (sender, receiver) = mpsc::channel(capacity);
    let metrics = Arc::new(QueueMetrics::new(name, capacity));

    (
        BoundedSender {
            sender,
            metrics: Arc::clone(&metrics),
        },
        BoundedReceiver { receiver, metrics },
    )
}
