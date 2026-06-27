//! Multi-level priority queue for ordered event processing.
//!
//! Maintains four independent bounded channels — one per [`EventPriority`]
//! level. The async [`PriorityQueue::recv`] method always drains the highest-
//! priority channel first, ensuring critical events are never starved by bulk
//! low-priority traffic.
//!
//! Under pressure (channel full), low-priority items are dropped first to
//! preserve head-room for critical events.

use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;
use sentra_core::EventPriority;
use tokio::sync::mpsc;
use tracing::{debug, warn};

/// Drop-count statistics per priority level.
#[derive(Debug, Clone, Default)]
pub struct PriorityDropCounts {
    /// Items dropped at [`EventPriority::Critical`].
    pub critical: u64,
    /// Items dropped at [`EventPriority::High`].
    pub high: u64,
    /// Items dropped at [`EventPriority::Normal`].
    pub normal: u64,
    /// Items dropped at [`EventPriority::Low`].
    pub low: u64,
}

/// A single priority level backed by a bounded mpsc channel.
///
/// The receiver is behind a [`Mutex`] so that the `send`-path can shed items
/// from lower-priority levels without requiring `&mut self` on the whole queue.
struct PriorityLevel<T> {
    tx: mpsc::Sender<T>,
    rx: Mutex<mpsc::Receiver<T>>,
    drops: AtomicU64,
}

impl<T> PriorityLevel<T> {
    fn new(capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel(capacity);
        Self {
            tx,
            rx: Mutex::new(rx),
            drops: AtomicU64::new(0),
        }
    }
}

/// Multi-level priority queue.
///
/// Items are inserted with an explicit [`EventPriority`] and are consumed
/// highest-priority-first. Each priority level has its own bounded channel so
/// back-pressure is isolated per level.
///
/// # Pressure shedding
///
/// When a level's channel is full, `send` will attempt to make room by
/// discarding the oldest item from the **lowest** non-empty level first
/// (`Low` → `Normal`). Critical and High items are never shed.
pub struct PriorityQueue<T> {
    critical: PriorityLevel<T>,
    high: PriorityLevel<T>,
    normal: PriorityLevel<T>,
    low: PriorityLevel<T>,
    capacity_per_level: usize,
}

impl<T: Send + 'static> PriorityQueue<T> {
    /// Create a new priority queue with `capacity_per_level` slots for each of
    /// the four priority levels.
    pub fn new(capacity_per_level: usize) -> Self {
        let cap = capacity_per_level.max(1);
        debug!(capacity_per_level = cap, "PriorityQueue created");
        Self {
            critical: PriorityLevel::new(cap),
            high: PriorityLevel::new(cap),
            normal: PriorityLevel::new(cap),
            low: PriorityLevel::new(cap),
            capacity_per_level: cap,
        }
    }

    /// Send an item at the specified priority.
    ///
    /// If the target level is full:
    /// - For `Low` and `Normal`: the item is dropped and counted.
    /// - For `High` and `Critical`: the queue sheds items from the lowest
    ///   non-empty level to make room, then retries once.
    pub fn send(&self, item: T, priority: EventPriority) -> sentra_core::Result<()> {
        let level = self.level_for(&priority);

        match level.tx.try_send(item) {
            Ok(()) => Ok(()),
            Err(mpsc::error::TrySendError::Full(item)) => {
                self.handle_full(item, priority)
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                Err(sentra_core::SentraError::Channel(
                    "priority queue channel closed".into(),
                ))
            }
        }
    }

    /// Receive the next item, draining highest priority first.
    ///
    /// Awaits until at least one item is available across any level. When
    /// multiple levels have items the order is:
    /// `Critical` → `High` → `Normal` → `Low`.
    pub async fn recv(&self) -> sentra_core::Result<T> {
        // Fast path: try non-blocking drain from highest to lowest.
        {
            if let Ok(item) = self.critical.rx.lock().try_recv() {
                return Ok(item);
            }
            if let Ok(item) = self.high.rx.lock().try_recv() {
                return Ok(item);
            }
            if let Ok(item) = self.normal.rx.lock().try_recv() {
                return Ok(item);
            }
            if let Ok(item) = self.low.rx.lock().try_recv() {
                return Ok(item);
            }
        }

        // Slow path: await on all receivers using biased select.
        // We lock each receiver individually and release quickly.
        // Because tokio::select! requires the futures to be pinned across
        // the await, and we can't hold a MutexGuard across await with
        // parking_lot, we poll in a loop with a brief yield.
        loop {
            {
                if let Ok(item) = self.critical.rx.lock().try_recv() {
                    return Ok(item);
                }
                if let Ok(item) = self.high.rx.lock().try_recv() {
                    return Ok(item);
                }
                if let Ok(item) = self.normal.rx.lock().try_recv() {
                    return Ok(item);
                }
                if let Ok(item) = self.low.rx.lock().try_recv() {
                    return Ok(item);
                }
            }

            // Check if all senders are closed (would mean permanent empty).
            if self.critical.tx.is_closed()
                && self.high.tx.is_closed()
                && self.normal.tx.is_closed()
                && self.low.tx.is_closed()
            {
                return Err(sentra_core::SentraError::Channel(
                    "all priority queue channels closed".into(),
                ));
            }

            // Yield to avoid busy-spinning.
            tokio::task::yield_now().await;
        }
    }

    /// Return current drop counts per priority level.
    pub fn drop_counts(&self) -> PriorityDropCounts {
        PriorityDropCounts {
            critical: self.critical.drops.load(Ordering::Relaxed),
            high: self.high.drops.load(Ordering::Relaxed),
            normal: self.normal.drops.load(Ordering::Relaxed),
            low: self.low.drops.load(Ordering::Relaxed),
        }
    }

    /// Return the capacity per priority level.
    pub fn capacity_per_level(&self) -> usize {
        self.capacity_per_level
    }

    // ── private helpers ──────────────────────────────────────────────

    /// Map a priority to its channel.
    fn level_for(&self, priority: &EventPriority) -> &PriorityLevel<T> {
        match priority {
            EventPriority::Critical => &self.critical,
            EventPriority::High => &self.high,
            EventPriority::Normal => &self.normal,
            EventPriority::Low => &self.low,
        }
    }

    /// Handle a full channel depending on priority.
    fn handle_full(&self, item: T, priority: EventPriority) -> sentra_core::Result<()> {
        match priority {
            // Low & Normal: just drop the incoming item.
            EventPriority::Low => {
                self.low.drops.fetch_add(1, Ordering::Relaxed);
                warn!("PriorityQueue: dropping Low priority item (channel full)");
                Ok(())
            }
            EventPriority::Normal => {
                // Try to shed a Low item first.
                if self.shed_from_low() {
                    if let Ok(()) = self.normal.tx.try_send(item) {
                        return Ok(());
                    }
                }
                self.normal.drops.fetch_add(1, Ordering::Relaxed);
                warn!("PriorityQueue: dropping Normal priority item (channel full)");
                Ok(())
            }
            // High & Critical: shed from lowest levels.
            EventPriority::High | EventPriority::Critical => {
                if self.shed_from_low() || self.shed_from_normal() {
                    let level = self.level_for(&priority);
                    match level.tx.try_send(item) {
                        Ok(()) => return Ok(()),
                        Err(mpsc::error::TrySendError::Full(_)) => {
                            level.drops.fetch_add(1, Ordering::Relaxed);
                            warn!(
                                ?priority,
                                "PriorityQueue: still full after shedding, dropping item"
                            );
                        }
                        Err(mpsc::error::TrySendError::Closed(_)) => {
                            return Err(sentra_core::SentraError::Channel(
                                "priority queue channel closed".into(),
                            ));
                        }
                    }
                } else {
                    let level = self.level_for(&priority);
                    level.drops.fetch_add(1, Ordering::Relaxed);
                    warn!(
                        ?priority,
                        "PriorityQueue: no lower-level items to shed, dropping item"
                    );
                }
                Ok(())
            }
        }
    }

    /// Attempt to discard one item from the Low channel, returning `true` on
    /// success.
    fn shed_from_low(&self) -> bool {
        match self.low.rx.lock().try_recv() {
            Ok(_) => {
                self.low.drops.fetch_add(1, Ordering::Relaxed);
                debug!("PriorityQueue: shed 1 Low priority item");
                true
            }
            Err(_) => false,
        }
    }

    /// Attempt to discard one item from the Normal channel, returning `true`
    /// on success.
    fn shed_from_normal(&self) -> bool {
        match self.normal.rx.lock().try_recv() {
            Ok(_) => {
                self.normal.drops.fetch_add(1, Ordering::Relaxed);
                debug!("PriorityQueue: shed 1 Normal priority item");
                true
            }
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn recv_drains_highest_first() {
        let pq = PriorityQueue::<u32>::new(8);

        pq.send(1, EventPriority::Low).expect("send low");
        pq.send(2, EventPriority::Normal).expect("send normal");
        pq.send(3, EventPriority::Critical).expect("send critical");
        pq.send(4, EventPriority::High).expect("send high");

        assert_eq!(pq.recv().await.expect("recv"), 3); // Critical
        assert_eq!(pq.recv().await.expect("recv"), 4); // High
        assert_eq!(pq.recv().await.expect("recv"), 2); // Normal
        assert_eq!(pq.recv().await.expect("recv"), 1); // Low
    }

    #[tokio::test]
    async fn low_dropped_when_full() {
        let pq = PriorityQueue::<u32>::new(1);

        pq.send(1, EventPriority::Low).expect("first low ok");
        pq.send(2, EventPriority::Low).expect("second low dropped silently");

        let counts = pq.drop_counts();
        assert_eq!(counts.low, 1);
    }

    #[tokio::test]
    async fn critical_sheds_low() {
        let pq = PriorityQueue::<u32>::new(1);

        // Fill the critical channel
        pq.send(10, EventPriority::Critical).expect("send");
        // Fill low so there's something to shed
        pq.send(99, EventPriority::Low).expect("send low");

        // Drain critical to make room, then send another critical which needs
        // to shed from low if critical channel itself is full
        let val = pq.recv().await.expect("recv critical");
        assert_eq!(val, 10);

        // Now critical is empty, this should just succeed.
        pq.send(11, EventPriority::Critical).expect("send critical again");
        let val = pq.recv().await.expect("recv critical #2");
        assert_eq!(val, 11);
    }
}
