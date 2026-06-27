use std::sync::atomic::{AtomicU64, Ordering};

pub struct ProcessMetrics {
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    pub enumeration_latency_us: AtomicU64,
    pub enrichment_latency_us: AtomicU64,
}

impl ProcessMetrics {
    pub const fn new() -> Self {
        Self {
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            enumeration_latency_us: AtomicU64::new(0),
            enrichment_latency_us: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn inc_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn inc_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }
}

pub static METRICS: ProcessMetrics = ProcessMetrics::new();
