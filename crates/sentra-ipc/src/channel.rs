//! Bounded channel system with health monitoring.
//!
//! Wraps [`tokio::sync::mpsc`] with atomic counters that track total sends,
//! receives, and drops. Both the sender and receiver halves expose a
//! [`ChannelHealth`] snapshot at any time without blocking the data path.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tokio::sync::mpsc;
use tracing::warn;

/// Shared counters between sender and receiver halves.
#[derive(Debug)]
struct SharedMetrics {
    name: String,
    capacity: usize,
    total_sent: AtomicU64,
    total_received: AtomicU64,
    total_dropped: AtomicU64,
}

/// Health snapshot for a [`BoundedChannel`].
#[derive(Debug, Clone)]
pub struct ChannelHealth {
    /// Human-readable name of the channel.
    pub name: String,
    /// Maximum capacity of the channel.
    pub capacity: usize,
    /// Approximate number of messages currently in the channel.
    pub pending: usize,
    /// Total messages successfully sent since creation.
    pub total_sent: u64,
    /// Total messages successfully received since creation.
    pub total_received: u64,
    /// Total messages dropped (send failures) since creation.
    pub total_dropped: u64,
    /// Approximate fill percentage (`pending / capacity * 100`).
    pub fill_percent: f32,
}

/// A bounded channel with built-in health monitoring.
///
/// This is a factory type — call [`BoundedChannel::new`] to get the sender and
/// receiver halves. The channel itself is not stored; the halves share
/// ownership of the metrics through an [`Arc`].
pub struct BoundedChannel;

impl BoundedChannel {
    /// Create a new bounded channel with the given `capacity` and `name`.
    ///
    /// Returns a `(ChannelSender<T>, ChannelReceiver<T>)` pair. Both halves
    /// share atomic counters so [`ChannelHealth`] is consistent regardless of
    /// which half you query.
    pub fn new<T>(capacity: usize, name: String) -> (ChannelSender<T>, ChannelReceiver<T>) {
        let (tx, rx) = mpsc::channel(capacity);
        let metrics = Arc::new(SharedMetrics {
            name,
            capacity,
            total_sent: AtomicU64::new(0),
            total_received: AtomicU64::new(0),
            total_dropped: AtomicU64::new(0),
        });

        let sender = ChannelSender {
            inner: tx,
            metrics: Arc::clone(&metrics),
        };
        let receiver = ChannelReceiver {
            inner: rx,
            metrics,
        };

        (sender, receiver)
    }
}

/// Sending half of a [`BoundedChannel`].
///
/// Applies back-pressure when the channel is full via the async `send` method.
/// A non-blocking `try_send` is also available that increments the drop counter
/// on failure.
#[derive(Debug)]
pub struct ChannelSender<T> {
    inner: mpsc::Sender<T>,
    metrics: Arc<SharedMetrics>,
}

impl<T> ChannelSender<T> {
    /// Send an item into the channel, waiting asynchronously if the channel is
    /// full (back-pressure).
    ///
    /// Increments `total_sent` on success or `total_dropped` on failure (e.g.
    /// receiver dropped).
    pub async fn send(&self, item: T) -> sentra_core::Result<()> {
        match self.inner.send(item).await {
            Ok(()) => {
                self.metrics.total_sent.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(_) => {
                self.metrics.total_dropped.fetch_add(1, Ordering::Relaxed);
                warn!(channel = %self.metrics.name, "send failed — receiver dropped");
                Err(sentra_core::SentraError::Channel(format!(
                    "channel '{}' receiver closed",
                    self.metrics.name
                )))
            }
        }
    }

    /// Attempt a non-blocking send.
    ///
    /// If the channel is full or the receiver has been dropped the item is
    /// counted as dropped and an error is returned.
    pub fn try_send(&self, item: T) -> sentra_core::Result<()> {
        match self.inner.try_send(item) {
            Ok(()) => {
                self.metrics.total_sent.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                self.metrics.total_dropped.fetch_add(1, Ordering::Relaxed);
                warn!(channel = %self.metrics.name, "try_send failed — channel full");
                Err(sentra_core::SentraError::Channel(format!(
                    "channel '{}' is full",
                    self.metrics.name
                )))
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                self.metrics.total_dropped.fetch_add(1, Ordering::Relaxed);
                warn!(channel = %self.metrics.name, "try_send failed — receiver dropped");
                Err(sentra_core::SentraError::Channel(format!(
                    "channel '{}' receiver closed",
                    self.metrics.name
                )))
            }
        }
    }

    /// Return a health snapshot of the channel.
    pub fn health(&self) -> ChannelHealth {
        build_health(&self.metrics, self.inner.capacity())
    }
}

impl<T> Clone for ChannelSender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

/// Receiving half of a [`BoundedChannel`].
#[derive(Debug)]
pub struct ChannelReceiver<T> {
    inner: mpsc::Receiver<T>,
    metrics: Arc<SharedMetrics>,
}

impl<T> ChannelReceiver<T> {
    /// Receive the next item from the channel, waiting asynchronously until
    /// one is available.
    ///
    /// Returns `None` when all senders have been dropped.
    pub async fn recv(&mut self) -> Option<T> {
        let item = self.inner.recv().await;
        if item.is_some() {
            self.metrics.total_received.fetch_add(1, Ordering::Relaxed);
        }
        item
    }

    /// Attempt a non-blocking receive.
    ///
    /// Returns `Ok(item)` if available, or an error if the channel is empty or
    /// closed.
    pub fn try_recv(&mut self) -> sentra_core::Result<T> {
        match self.inner.try_recv() {
            Ok(item) => {
                self.metrics.total_received.fetch_add(1, Ordering::Relaxed);
                Ok(item)
            }
            Err(mpsc::error::TryRecvError::Empty) => Err(sentra_core::SentraError::Channel(
                format!("channel '{}' is empty", self.metrics.name),
            )),
            Err(mpsc::error::TryRecvError::Disconnected) => Err(sentra_core::SentraError::Channel(
                format!("channel '{}' senders dropped", self.metrics.name),
            )),
        }
    }

    /// Return a health snapshot of the channel.
    pub fn health(&self) -> ChannelHealth {
        // mpsc::Receiver doesn't expose current len; derive from counters.
        build_health(&self.metrics, self.metrics.capacity)
    }
}

/// Build a [`ChannelHealth`] snapshot from the shared metrics.
fn build_health(metrics: &SharedMetrics, max_capacity: usize) -> ChannelHealth {
    let sent = metrics.total_sent.load(Ordering::Relaxed);
    let received = metrics.total_received.load(Ordering::Relaxed);
    let dropped = metrics.total_dropped.load(Ordering::Relaxed);
    let pending = sent.saturating_sub(received) as usize;
    let fill_percent = if max_capacity > 0 {
        (pending as f32 / max_capacity as f32) * 100.0
    } else {
        0.0
    };

    ChannelHealth {
        name: metrics.name.clone(),
        capacity: metrics.capacity,
        pending,
        total_sent: sent,
        total_received: received,
        total_dropped: dropped,
        fill_percent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_and_receive() {
        let (tx, mut rx) = BoundedChannel::new::<u32>(8, "test".into());
        tx.send(42).await.expect("send should succeed");

        let val = rx.recv().await.expect("should receive");
        assert_eq!(val, 42);

        let health = tx.health();
        assert_eq!(health.total_sent, 1);
        assert_eq!(health.total_dropped, 0);

        let rh = rx.health();
        assert_eq!(rh.total_received, 1);
    }

    #[tokio::test]
    async fn try_send_full_channel() {
        let (tx, _rx) = BoundedChannel::new::<u32>(1, "tiny".into());
        // Fill the channel
        tx.try_send(1).expect("first try_send should work");
        // Second should fail — channel full
        let result = tx.try_send(2);
        assert!(result.is_err());
        assert_eq!(tx.health().total_dropped, 1);
    }

    #[tokio::test]
    async fn health_fill_percent() {
        let (tx, _rx) = BoundedChannel::new::<u32>(4, "fill-test".into());
        tx.send(1).await.expect("send");
        tx.send(2).await.expect("send");

        let h = tx.health();
        assert_eq!(h.pending, 2);
        // 2/4 = 50%
        assert!((h.fill_percent - 50.0).abs() < f32::EPSILON);
    }

    #[tokio::test]
    async fn sender_cloneable() {
        let (tx, mut rx) = BoundedChannel::new::<u32>(8, "clone-test".into());
        let tx2 = tx.clone();
        tx.send(1).await.expect("send");
        tx2.send(2).await.expect("send via clone");

        assert_eq!(rx.recv().await, Some(1));
        assert_eq!(rx.recv().await, Some(2));
        assert_eq!(rx.health().total_received, 2);
    }
}
