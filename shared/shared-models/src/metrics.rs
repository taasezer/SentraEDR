use std::sync::atomic::{AtomicU64, Ordering};

/// Generic reusable metrics infrastructure for engine tracking.
/// Note: Engines must instantiate their own counters; these are the building blocks.
pub struct InfrastructureMetrics {
    pub events_processed: AtomicU64,
    pub processing_failures: AtomicU64,
    pub dropped_events: AtomicU64,
    pub queue_depth: AtomicU64,
    pub queue_overflow_count: AtomicU64,
}

impl InfrastructureMetrics {
    pub const fn new() -> Self {
        Self {
            events_processed: AtomicU64::new(0),
            processing_failures: AtomicU64::new(0),
            dropped_events: AtomicU64::new(0),
            queue_depth: AtomicU64::new(0),
            queue_overflow_count: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn inc_processed(&self) { self.events_processed.fetch_add(1, Ordering::Relaxed); }
    
    #[inline(always)]
    pub fn inc_failures(&self) { self.processing_failures.fetch_add(1, Ordering::Relaxed); }
    
    #[inline(always)]
    pub fn inc_overflows(&self) { self.queue_overflow_count.fetch_add(1, Ordering::Relaxed); }
}
