use shared_models::events::{NormalizedTelemetryEvent, EventType};
use crate::models::{ProcessIdentity, ProcessMetadata, ProcessSnapshot};
use crate::metrics::METRICS;
use std::collections::HashMap;

/// An extremely simplified LRU/Cache mapping PID to known static metadata.
/// In production, this tracks tombstones and enforces TTL eviction.
pub struct ProcessCache {
    entries: HashMap<u32, ProcessMetadata>,
}

impl ProcessCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pid: u32, meta: ProcessMetadata) {
        self.entries.insert(pid, meta);
    }

    pub fn get(&self, pid: u32) -> Option<&ProcessMetadata> {
        self.entries.get(&pid)
    }
    
    pub fn tombstone(&mut self, pid: u32) {
        self.entries.remove(&pid);
    }
}

/// The core enrichment loop. Consumes purely immutable events.
pub struct ProcessAnalyzer {
    cache: ProcessCache,
}

impl ProcessAnalyzer {
    pub fn new() -> Self {
        Self {
            cache: ProcessCache::new(),
        }
    }

    /// Process an immutable telemetry event, querying cache or OS, and yielding a Snapshot.
    pub fn process_event(&mut self, event: &NormalizedTelemetryEvent) -> Option<ProcessSnapshot> {
        // Determine identity
        let identity = ProcessIdentity {
            pid: event.process_id,
            creation_time_ms: event.timestamp_ms,
            original_event_id: event.event_id,
        };

        // Cache Management
        match &event.event_type {
            EventType::ProcessCreate { image_path, command_line } => {
                let meta = ProcessMetadata {
                    image_path: image_path.clone(),
                    command_line: command_line.clone(),
                    session_id: 0, // Mocked OS query
                    initial_user_sid: "S-1-5-18".to_string(), // Mocked OS query
                };
                self.cache.insert(event.process_id, meta);
            }
            EventType::ProcessExit { .. } => {
                self.cache.tombstone(event.process_id);
                return None; // No active snapshot after exit
            }
            _ => {}
        }

        // Enrichment
        let metadata = if let Some(cached) = self.cache.get(event.process_id) {
            METRICS.inc_hit();
            cached.clone()
        } else {
            METRICS.inc_miss();
            // In a real system, this triggers `ProcessQuerySource::open_process` 
            // and retrieves the metadata dynamically.
            ProcessMetadata {
                image_path: "UNKNOWN".to_string(),
                command_line: "UNKNOWN".to_string(),
                session_id: 0,
                initial_user_sid: "UNKNOWN".to_string(),
            }
        };

        Some(ProcessSnapshot {
            identity,
            parent_identity: None,
            metadata,
            current_integrity_level: "Medium".to_string(), // Mocked token query
        })
    }
}
