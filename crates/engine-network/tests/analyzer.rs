use engine_network::{NetworkAnalyzer, SignalSeverity};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata, TelemetrySource,
    Timestamp,
};

fn ts(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}

fn net_event(
    ip: &str,
    port: &str,
    domain: Option<&str>,
    observed_at: &str,
) -> NormalizedTelemetryEvent {
    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Medium,
        TelemetryAction::NetworkConnectionObserved,
        ts(observed_at),
    );
    let mut metadata = TelemetryMetadata::empty()
        .insert("network.remote_ip", ip)
        .insert("network.remote_port", port)
        .insert("network.protocol", "tcp")
        .insert("network.direction", "outbound");
    if let Some(domain) = domain {
        metadata = metadata.insert("network.domain", domain);
    }
    event.metadata = metadata;
    event
}

#[test]
fn public_ip_without_domain_emits_rare_and_ip_literal_signals() {
    let mut analyzer = NetworkAnalyzer::default();
    let report = analyzer.analyze(net_event(
        "203.0.113.10",
        "443",
        None,
        "2026-06-27T09:00:00Z",
    ));

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 1);
    assert_eq!(report.tracked_destinations, 1);
    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "rare_external_destination")
    );
    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "ip_literal_connection")
    );
}

#[test]
fn suspicious_domain_and_high_risk_port_emit_signals() {
    let mut analyzer = NetworkAnalyzer::default();
    let report = analyzer.analyze(net_event(
        "198.51.100.5",
        "4444",
        Some("a.b.c.d.e.duckdns.org"),
        "2026-06-27T09:00:00Z",
    ));

    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "suspicious_dns_pattern")
    );
    let port_signal = report
        .signals
        .iter()
        .find(|s| s.name == "high_risk_port")
        .unwrap();
    assert_eq!(port_signal.severity, SignalSeverity::Medium);
}

#[test]
fn repeated_equal_intervals_emit_beacon_candidate() {
    let mut analyzer = NetworkAnalyzer::default();
    analyzer.analyze(net_event(
        "198.51.100.7",
        "443",
        Some("node.example.net"),
        "2026-06-27T09:00:00Z",
    ));
    analyzer.analyze(net_event(
        "198.51.100.7",
        "443",
        Some("node.example.net"),
        "2026-06-27T09:01:00Z",
    ));
    let report = analyzer.analyze(net_event(
        "198.51.100.7",
        "443",
        Some("node.example.net"),
        "2026-06-27T09:02:00Z",
    ));

    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "beacon_interval_candidate")
    );
}

#[test]
fn private_destination_is_not_rare_external() {
    let mut analyzer = NetworkAnalyzer::default();
    let report = analyzer.analyze(net_event(
        "192.168.1.10",
        "443",
        None,
        "2026-06-27T09:00:00Z",
    ));

    assert!(
        !report
            .signals
            .iter()
            .any(|s| s.name == "rare_external_destination")
    );
}

#[test]
fn irrelevant_telemetry_is_ignored() {
    let mut analyzer = NetworkAnalyzer::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::InternalHealth,
        EventPriority::Low,
        TelemetryAction::ComponentHealthChanged,
        ts("2026-06-27T09:00:00Z"),
    );
    let report = analyzer.analyze(event);

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 0);
    assert_eq!(report.stats.ignored, 1);
    assert!(report.signals.is_empty());
}
