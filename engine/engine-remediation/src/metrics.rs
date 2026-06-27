use std::sync::atomic::{AtomicU64, Ordering};

pub struct RemediationMetrics {
    pub planning_latency_us: AtomicU64,
    pub execution_latency_us: AtomicU64,
    pub rollback_creation_latency_us: AtomicU64,
    pub verification_latency_us: AtomicU64,
    pub provider_overhead_us: AtomicU64,
}

impl RemediationMetrics {
    pub const fn new() -> Self {
        Self {
            planning_latency_us: AtomicU64::new(0),
            execution_latency_us: AtomicU64::new(0),
            rollback_creation_latency_us: AtomicU64::new(0),
            verification_latency_us: AtomicU64::new(0),
            provider_overhead_us: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn track_exec_latency(&self, latency: u64) {
        self.execution_latency_us.fetch_add(latency, Ordering::Relaxed);
    }
}

pub static METRICS: RemediationMetrics = RemediationMetrics::new();
