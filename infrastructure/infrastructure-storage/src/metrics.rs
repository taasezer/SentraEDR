use std::sync::atomic::{AtomicU64, Ordering};

pub struct StorageMetrics {
    pub batch_flush_latency_us: AtomicU64,
    pub events_persisted_total: AtomicU64,
    pub storage_queue_depth: AtomicU64,
}

impl StorageMetrics {
    pub const fn new() -> Self {
        Self {
            batch_flush_latency_us: AtomicU64::new(0),
            events_persisted_total: AtomicU64::new(0),
            storage_queue_depth: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn track_flush(&self, latency: u64, count: u64) {
        self.batch_flush_latency_us
            .store(latency, Ordering::Relaxed);
        self.events_persisted_total
            .fetch_add(count, Ordering::Relaxed);
    }
}

pub static METRICS: StorageMetrics = StorageMetrics::new();
