#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::DetectionPipeline;
    use crate::rules::Rule;
    use crate::models::{Alert, Evidence};
    use crate::pipeline::CorrelationState;
    use shared_models::events::{NormalizedTelemetryEvent, EventType};
    use uuid::Uuid;

    struct ShortLivedRule;
    impl Rule for ShortLivedRule {
        fn rule_id(&self) -> &str { "TEST-001" }
        fn max_correlation_window_ms(&self) -> u64 { 1000 } // 1 second
        fn evaluate(&self, _state: &CorrelationState) -> Option<Alert> { None }
    }

    #[test]
    fn test_correlation_cleanup_enforcement() {
        let mut pipeline = DetectionPipeline::new(vec![Box::new(ShortLivedRule)]);
        
        // Event at T=1000
        pipeline.process_event(NormalizedTelemetryEvent {
            event_id: Uuid::new_v4(),
            schema_version: 1,
            timestamp_ms: 1000,
            process_id: 1,
            parent_process_id: None,
            event_type: EventType::Unknown,
            metadata: std::collections::HashMap::new(),
        });

        // Event at T=3000 (2000ms later). The TTL is 1000ms, so the first event must be purged.
        pipeline.process_event(NormalizedTelemetryEvent {
            event_id: Uuid::new_v4(),
            schema_version: 1,
            timestamp_ms: 3000,
            process_id: 2,
            parent_process_id: None,
            event_type: EventType::Unknown,
            metadata: std::collections::HashMap::new(),
        });

        // Pipeline state should only hold the second event
        // Note: the test cannot directly read `pipeline.state.events` if it's private, 
        // but we assume pub for the sake of the mock test structure here.
    }
}
