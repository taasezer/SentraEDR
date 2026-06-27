use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity, RiskLevel,
    TelemetryAction, TelemetrySource, Timestamp,
};

#[test]
fn telemetry_event_roundtrips_through_json() {
    let timestamp = Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap();
    let process = ProcessIdentity::new(4242)
        .with_parent(1000)
        .with_image_path(ImagePath::new(
            r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        ))
        .with_command_line(CommandLine::new("powershell.exe -NoProfile"));

    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::High,
        TelemetryAction::PowerShellExecuted,
        timestamp,
    )
    .with_process(process)
    .with_confidence_hint(80);

    let encoded = serde_json::to_string(&event).unwrap();
    let decoded: NormalizedTelemetryEvent = serde_json::from_str(&encoded).unwrap();

    assert_eq!(decoded.schema_version.major, 1);
    assert_eq!(decoded.schema_version.minor, 0);
    assert_eq!(decoded.priority, EventPriority::High);
    assert_eq!(decoded.confidence_hint, 80);
}

#[test]
fn finding_score_is_clamped_to_100() {
    let timestamp = Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap();
    let finding = shared_models::Finding::new(timestamp, RiskLevel::Critical, 200);

    assert_eq!(finding.score, 100);
}
