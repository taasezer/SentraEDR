#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::NetworkAnalyzer;
    use shared_models::events::{EventType, NormalizedTelemetryEvent};
    use uuid::Uuid;

    #[test]
    fn test_dns_enrichment_flow() {
        let mut analyzer = NetworkAnalyzer::new();

        // Pre-seed the cache simulating a prior DNS response event.
        analyzer
            .dns_cache
            .insert("8.8.8.8".to_string(), "dns.google".to_string());

        let event = NormalizedTelemetryEvent {
            event_id: Uuid::new_v4(),
            schema_version: 1,
            timestamp_ms: 1000,
            process_id: 4444,
            parent_process_id: None,
            event_type: EventType::NetworkConnection {
                destination_ip: "8.8.8.8".to_string(),
                destination_port: 443,
                protocol: "TCP".to_string(),
            },
            metadata: std::collections::HashMap::new(),
        };

        let result = analyzer.process_event(&event);
        assert!(result.is_some());

        let (snapshot, state) = result.unwrap();

        // Prove IPv4 and IP parsing
        assert!(!snapshot.metadata.is_ipv6);
        assert!(!snapshot.metadata.is_loopback);

        // Prove DNS Cache Hit
        assert_eq!(snapshot.resolved_hostname.unwrap(), "dns.google");
    }
}
