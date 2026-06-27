use crate::{EtwProviderStatus, get_default_providers};
use chrono::{DateTime, Utc};
use sentra_core::{RawInputEvent, TelemetryEvent};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct RawEtwEvent {
    pub provider_guid: String,
    pub event_id: u16,
    pub timestamp: DateTime<Utc>,
    pub process_id: u32,
    pub thread_id: u32,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PipelineHealth {
    pub events_processed: u64,
    pub events_normalized: u64,
    pub events_dropped: u64,
    pub events_per_second: f64,
    pub last_event_time: Option<DateTime<Utc>>,
    pub provider_statuses: Vec<EtwProviderStatus>,
}

pub struct TelemetryPipeline {
    events_processed: AtomicU64,
    events_normalized: AtomicU64,
    events_dropped: AtomicU64,
}

impl TelemetryPipeline {
    pub fn new() -> Self {
        Self {
            events_processed: AtomicU64::new(0),
            events_normalized: AtomicU64::new(0),
            events_dropped: AtomicU64::new(0),
        }
    }

    pub fn process_raw_event(&self, provider_name: &str, event_data: RawEtwEvent) -> Option<TelemetryEvent> {
        self.events_processed.fetch_add(1, Ordering::Relaxed);
        
        // This is a stub for the normalization logic.
        // A real implementation parses the ETW payload bytes based on MOF/WPP schemas.
        // For now, we wrap it in a RawInputEvent.
        
        self.events_normalized.fetch_add(1, Ordering::Relaxed);
        Some(TelemetryEvent::RawInput(RawInputEvent {
            source: provider_name.to_string(),
            payload: format!("Event {} from process {}", event_data.event_id, event_data.process_id),
            timestamp: event_data.timestamp,
        }))
    }

    pub fn health(&self) -> PipelineHealth {
        let statuses = get_default_providers()
            .into_iter()
            .map(|p| EtwProviderStatus {
                provider: p,
                is_active: false,
                events_received: 0,
                events_lost: 0,
                last_event_time: None,
            })
            .collect();

        PipelineHealth {
            events_processed: self.events_processed.load(Ordering::Relaxed),
            events_normalized: self.events_normalized.load(Ordering::Relaxed),
            events_dropped: self.events_dropped.load(Ordering::Relaxed),
            events_per_second: 0.0,
            last_event_time: Some(Utc::now()),
            provider_statuses: statuses,
        }
    }
}
