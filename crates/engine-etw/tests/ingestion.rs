use engine_etw::{EtwIngestor, EtwProcessEventKind, EtwProcessRecord, SyntheticEtwSource};
use shared_ipc::bounded_channel;
use shared_models::{HealthStatus, NormalizedTelemetryEvent, TelemetryAction, Timestamp};

#[test]
fn synthetic_source_drains_into_bounded_queue() {
    let records = vec![
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
            42,
        ),
        EtwProcessRecord::new(
            EtwProcessEventKind::Exit,
            Timestamp::parse_rfc3339("2026-06-27T09:01:00Z").unwrap(),
            42,
        ),
    ];
    let source = SyntheticEtwSource::from_records(records);
    let (sender, mut receiver) = bounded_channel::<NormalizedTelemetryEvent>("etw-process", 4);

    let report = EtwIngestor::new(source, sender).drain();

    assert_eq!(report.stats.received, 2);
    assert_eq!(report.stats.normalized, 2);
    assert_eq!(report.stats.dropped, 0);
    assert_eq!(report.component_health.status, HealthStatus::Healthy);

    let first = receiver.try_recv().unwrap();
    let second = receiver.try_recv().unwrap();
    assert_eq!(first.action, TelemetryAction::ProcessStarted);
    assert_eq!(second.action, TelemetryAction::ProcessExited);
}

#[test]
fn queue_pressure_degrades_component_health() {
    let records = vec![
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
            1,
        ),
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:01Z").unwrap(),
            2,
        ),
    ];
    let source = SyntheticEtwSource::from_records(records);
    let (sender, _receiver) = bounded_channel::<NormalizedTelemetryEvent>("etw-process", 1);

    let report = EtwIngestor::new(source, sender).drain();

    assert_eq!(report.stats.received, 2);
    assert_eq!(report.stats.normalized, 1);
    assert_eq!(report.stats.dropped, 1);
    assert_eq!(report.component_health.status, HealthStatus::Degraded);
    assert_eq!(report.component_health.queue.unwrap().dropped_events, 1);
}
