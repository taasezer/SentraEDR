use std::sync::atomic::{AtomicU64, Ordering};

/// Global, thread-safe metrics for ETW ingestion health.
/// These are implemented as lock-free atomics to ensure zero contention in the hot path.
pub struct EtwMetrics {
    pub events_received: AtomicU64,
    pub events_parsed: AtomicU64,
    pub events_normalized: AtomicU64,
    pub parser_failures: AtomicU64,
    pub normalization_failures: AtomicU64,
    pub dropped_events: AtomicU64,
    pub queue_depth: AtomicU64,
    pub queue_overflow_count: AtomicU64,
}

impl EtwMetrics {
    pub const fn new() -> Self {
        Self {
            events_received: AtomicU64::new(0),
            events_parsed: AtomicU64::new(0),
            events_normalized: AtomicU64::new(0),
            parser_failures: AtomicU64::new(0),
            normalization_failures: AtomicU64::new(0),
            dropped_events: AtomicU64::new(0),
            queue_depth: AtomicU64::new(0),
            queue_overflow_count: AtomicU64::new(0),
        }
    }
}

// Global static metrics instance.
pub static METRICS: EtwMetrics = EtwMetrics::new();

/// Convenience functions for the hot path to keep code clean.
#[inline(always)]
pub fn inc_received() { METRICS.events_received.fetch_add(1, Ordering::Relaxed); }

#[inline(always)]
pub fn inc_parsed() { METRICS.events_parsed.fetch_add(1, Ordering::Relaxed); }

#[inline(always)]
pub fn inc_parser_failure() { METRICS.parser_failures.fetch_add(1, Ordering::Relaxed); }

#[inline(always)]
pub fn inc_normalized() { METRICS.events_normalized.fetch_add(1, Ordering::Relaxed); }

#[inline(always)]
pub fn inc_normalization_failure() { METRICS.normalization_failures.fetch_add(1, Ordering::Relaxed); }

#[inline(always)]
pub fn inc_overflow() { METRICS.queue_overflow_count.fetch_add(1, Ordering::Relaxed); }
