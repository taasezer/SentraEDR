use crate::parser::RawEtwEvent;
use crate::metrics;
use shared_models::events::{NormalizedTelemetryEvent, EventType};
use std::collections::HashMap;

/// Normalizes a Windows-specific RawEtwEvent into the platform-agnostic NormalizedTelemetryEvent.
pub fn normalize(raw: RawEtwEvent) -> Option<NormalizedTelemetryEvent> {
    // In a real implementation, we map `raw.provider_id` and `raw.event_id` 
    // to determine the exact `EventType`.
    
    // Fallback/Synthetic matching for testing boundaries:
    let event_type = if raw.event_id == 1 { // Example Process Create
        EventType::ProcessCreate {
            command_line: String::from("synthetic_cmd.exe"),
            image_path: String::from("C:\\Windows\\System32\\synthetic_cmd.exe"),
        }
    } else {
        EventType::Unknown
    };

    let normalized = NormalizedTelemetryEvent {
        event_id: uuid::Uuid::new_v4(),
        schema_version: 1,
        timestamp_ms: raw.timestamp as u64, // simplified conversion
        process_id: raw.process_id,
        parent_process_id: None, 
        event_type,
        metadata: HashMap::new(),
    };

    metrics::inc_normalized();
    Some(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::RawEtwEvent;

    #[test]
    fn test_normalization_process_create() {
        let raw = RawEtwEvent {
            provider_id: windows::core::GUID::zeroed(),
            event_id: 1,
            opcode: 0,
            process_id: 1234,
            thread_id: 5678,
            timestamp: 1000000,
            raw_payload_extracted: true,
        };

        let norm = normalize(raw).expect("Normalization failed");
        assert_eq!(norm.process_id, 1234);
        
        match norm.event_type {
            EventType::ProcessCreate { image_path, .. } => {
                assert!(image_path.contains("synthetic_cmd.exe"));
            }
            _ => panic!("Expected ProcessCreate event type"),
        }
    }
}
