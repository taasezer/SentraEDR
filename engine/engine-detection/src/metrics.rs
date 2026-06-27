use std::sync::atomic::{AtomicU64, Ordering};

pub struct DetectionMetrics {
    pub correlation_latency_us: AtomicU64,
    pub rule_evaluation_latency_us: AtomicU64,
    pub alert_generation_latency_us: AtomicU64,
    pub active_correlation_count: AtomicU64,
    pub correlation_memory_bytes: AtomicU64,
}

impl DetectionMetrics {
    pub const fn new() -> Self {
        Self {
            correlation_latency_us: AtomicU64::new(0),
            rule_evaluation_latency_us: AtomicU64::new(0),
            alert_generation_latency_us: AtomicU64::new(0),
            active_correlation_count: AtomicU64::new(0),
            correlation_memory_bytes: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn track_active_correlations(&self, count: u64) {
        self.active_correlation_count.store(count, Ordering::Relaxed);
    }
}

pub static METRICS: DetectionMetrics = DetectionMetrics::new();
