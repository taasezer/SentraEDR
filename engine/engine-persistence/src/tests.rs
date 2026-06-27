#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::PersistenceAnalyzer;
    use crate::source::PersistenceProvider;
    use crate::models::{PersistenceIdentity, PersistenceSnapshot, PersistenceMetadata};
    use crate::errors::PersistenceEngineError;
    use shared_models::events::{NormalizedTelemetryEvent, EventType};
    use uuid::Uuid;

    struct MockProvider;

    impl PersistenceProvider for MockProvider {
        fn provider_type(&self) -> &str { "Mock" }
        fn query(&self, identity: &PersistenceIdentity) -> Result<Option<PersistenceSnapshot>, PersistenceEngineError> {
            Ok(Some(PersistenceSnapshot {
                identity: identity.clone(),
                metadata: PersistenceMetadata {
                    location_path: "mock_path".to_string(),
                    target_binary_path: "mock.exe".to_string(),
                    author: None,
                },
                is_active: true,
                snapshot_id: Uuid::new_v4(),
            }))
        }
        fn list_all(&self) -> Result<Vec<PersistenceSnapshot>, PersistenceEngineError> {
            Ok(vec![])
        }
    }

    #[test]
    fn test_modular_provider_routing() {
        let mut analyzer = PersistenceAnalyzer::new(vec![Box::new(MockProvider)]);
        let event = NormalizedTelemetryEvent {
            event_id: Uuid::new_v4(),
            schema_version: 1,
            timestamp_ms: 1000,
            process_id: 1,
            parent_process_id: None,
            event_type: EventType::Unknown,
            metadata: std::collections::HashMap::new(),
        };
        
        let change = analyzer.process_event(&event);
        assert!(change.is_some());
    }
}
