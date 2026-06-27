//! Internal event bus for fan-out distribution of telemetry events.
//!
//! Uses `tokio::sync::broadcast` to deliver each published event to all active
//! subscribers. The bus is lock-free on the publish path and designed for high
//! throughput within a single process.

use sentra_core::TelemetryEvent;
use tokio::sync::broadcast;
use tracing::{debug, warn};

/// Default broadcast channel capacity when none is specified.
const DEFAULT_CAPACITY: usize = 4096;

/// Health snapshot of the [`EventBus`].
#[derive(Debug, Clone)]
pub struct BusHealth {
    /// Number of active subscribers (receivers that have not been dropped).
    pub subscriber_count: usize,
    /// Total capacity of the underlying broadcast channel.
    pub capacity: usize,
}

/// Central publish/subscribe bus for [`TelemetryEvent`]s.
///
/// Each call to [`EventBus::subscribe`] creates a new receiver that will see
/// every event published *after* the subscription was created. Slow receivers
/// that fall behind by more than `capacity` messages will experience lag (the
/// oldest unseen messages are dropped for that receiver).
pub struct EventBus {
    sender: broadcast::Sender<TelemetryEvent>,
    capacity: usize,
}

impl EventBus {
    /// Create a new event bus with the given broadcast capacity.
    ///
    /// A capacity of `0` is replaced with [`DEFAULT_CAPACITY`].
    pub fn new(capacity: usize) -> Self {
        let cap = if capacity == 0 { DEFAULT_CAPACITY } else { capacity };
        let (sender, _) = broadcast::channel(cap);
        debug!(capacity = cap, "EventBus created");
        Self {
            sender,
            capacity: cap,
        }
    }

    /// Publish a [`TelemetryEvent`] to all current subscribers.
    ///
    /// This is a non-blocking operation. Returns an error only if there are
    /// **zero** active receivers (all subscriptions have been dropped).
    pub fn publish(&self, event: TelemetryEvent) -> sentra_core::Result<()> {
        self.sender.send(event).map_err(|_| {
            warn!("EventBus publish failed: no active receivers");
            sentra_core::SentraError::Channel("EventBus has no active receivers".into())
        })?;
        Ok(())
    }

    /// Create a new subscription to the event bus.
    ///
    /// The returned receiver will observe every event published *after* this
    /// call. If the receiver falls behind by more than `capacity` messages the
    /// oldest unseen messages are silently dropped for that receiver.
    pub fn subscribe(&self) -> broadcast::Receiver<TelemetryEvent> {
        let rx = self.sender.subscribe();
        debug!(
            subscriber_count = self.sender.receiver_count(),
            "New EventBus subscriber"
        );
        rx
    }

    /// Return a snapshot of the bus health.
    pub fn health(&self) -> BusHealth {
        BusHealth {
            subscriber_count: self.sender.receiver_count(),
            capacity: self.capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn publish_and_receive() {
        let bus = EventBus::new(16);
        let mut rx = bus.subscribe();

        let event = TelemetryEvent::default();
        bus.publish(event.clone()).expect("should publish");

        let received = rx.recv().await.expect("should receive");
        assert_eq!(received.id, event.id);
    }

    #[test]
    fn health_tracks_subscribers() {
        let bus = EventBus::new(16);
        assert_eq!(bus.health().subscriber_count, 0);

        let _rx1 = bus.subscribe();
        assert_eq!(bus.health().subscriber_count, 1);

        let _rx2 = bus.subscribe();
        assert_eq!(bus.health().subscriber_count, 2);

        drop(_rx1);
        assert_eq!(bus.health().subscriber_count, 1);
    }

    #[test]
    fn publish_without_receivers_errors() {
        let bus = EventBus::new(16);
        let result = bus.publish(TelemetryEvent::default());
        assert!(result.is_err());
    }

    #[test]
    fn default_capacity_used_when_zero() {
        let bus = EventBus::new(0);
        assert_eq!(bus.health().capacity, DEFAULT_CAPACITY);
    }
}
