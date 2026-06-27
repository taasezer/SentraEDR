use std::sync::atomic::{AtomicU64, Ordering};

pub struct PersistenceMetrics {
    pub registry_enumeration_latency_us: AtomicU64,
    pub snapshot_generation_latency_us: AtomicU64,
    pub snapshot_comparison_latency_us: AtomicU64,
    pub provider_execution_time_us: AtomicU64,
}

impl PersistenceMetrics {
    pub const fn new() -> Self {
        Self {
            registry_enumeration_latency_us: AtomicU64::new(0),
            snapshot_generation_latency_us: AtomicU64::new(0),
            snapshot_comparison_latency_us: AtomicU64::new(0),
            provider_execution_time_us: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn record_comparison_latency(&self, _latency: u64) {
        // In a real app this would use relaxed atomic max/avg or histograms.
        self.snapshot_comparison_latency_us.fetch_add(1, Ordering::Relaxed);
    }
}

pub static METRICS: PersistenceMetrics = PersistenceMetrics::new();
