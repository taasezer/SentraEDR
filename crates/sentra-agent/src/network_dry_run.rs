use engine_network::{NetworkAnalysisReport, NetworkAnalyzer};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata, TelemetrySource,
    Timestamp,
};

pub fn run_synthetic_network_analysis_dry_run() -> NetworkAnalysisReport {
    let mut analyzer = NetworkAnalyzer::default();

    analyzer.analyze(network_event("198.51.100.7", "4444", Some("node.duckdns.org"), "2026-06-27T09:04:00Z"));
    analyzer.analyze(network_event("198.51.100.7", "4444", Some("node.duckdns.org"), "2026-06-27T09:05:00Z"));
    analyzer.analyze(network_event("198.51.100.7", "4444", Some("node.duckdns.org"), "2026-06-27T09:06:00Z"))
}

fn network_event(
    remote_ip: &str,
    remote_port: &str,
    domain: Option<&str>,
    observed_at: &str,
) -> NormalizedTelemetryEvent {
    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Medium,
        TelemetryAction::NetworkConnectionObserved,
        Timestamp::parse_rfc3339(observed_at).unwrap(),
    );
    let mut metadata = TelemetryMetadata::empty()
        .insert("network.remote_ip", remote_ip)
        .insert("network.remote_port", remote_port)
        .insert("network.protocol", "tcp")
        .insert("network.direction", "outbound");
    if let Some(domain) = domain {
        metadata = metadata.insert("network.domain", domain);
    }
    event.metadata = metadata;
    event
}
