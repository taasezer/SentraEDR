#[cfg(test)]
mod tests {
    use crate::analyzer::ProcessAnalyzer;
    use shared_models::events::{EventType, NormalizedTelemetryEvent};
    use uuid::Uuid;

    #[test]
    fn test_cache_hit_and_immutability() {
        let mut analyzer = ProcessAnalyzer::new();

        let create_event = NormalizedTelemetryEvent {
            event_id: Uuid::new_v4(),
            schema_version: 1,
            timestamp_ms: 1000,
            process_id: 1234,
            parent_process_id: Some(100),
            event_type: EventType::ProcessCreate {
                command_line: "test.exe".to_string(),
                image_path: "C:\\test.exe".to_string(),
            },
            metadata: std::collections::HashMap::new(),
        };

        // Process creation populates the cache
        let snap1 = analyzer.process_event(&create_event).unwrap();
        assert_eq!(snap1.metadata.image_path, "C:\\test.exe");

        // Simulating a subsequent event on the same PID (e.g. registry read)
        let reg_event = NormalizedTelemetryEvent {
            event_id: Uuid::new_v4(),
            schema_version: 1,
            timestamp_ms: 1005,
            process_id: 1234,
            parent_process_id: None,
            event_type: EventType::RegistryActivity {
                key_path: "HKCU\\Software".to_string(),
                value_name: "Run".to_string(),
                action: "Read".to_string(),
            },
            metadata: std::collections::HashMap::new(),
        };

        // Cache hit proves the static metadata is correctly retrieved
        let snap2 = analyzer.process_event(&reg_event).unwrap();
        assert_eq!(snap2.metadata.image_path, "C:\\test.exe");

        // The original `create_event` and `reg_event` remain fully immutable and untouched.
    }
}
