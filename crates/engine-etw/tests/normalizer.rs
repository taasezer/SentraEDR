use engine_etw::{EtwProcessEventKind, EtwProcessRecord, normalize_process_record};
use shared_models::{EventPriority, TelemetryAction, TelemetrySource, Timestamp};

#[test]
fn process_start_record_normalizes_to_telemetry_event() {
    let record = EtwProcessRecord::new(
        EtwProcessEventKind::Start,
        Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
        4242,
    )
    .with_parent_process_id(1000)
    .with_image_path(r"C:\Windows\System32\cmd.exe")
    .with_command_line("cmd.exe /c whoami")
    .with_confidence(250);

    let event = normalize_process_record(record);
    let process = event.process.unwrap();

    assert_eq!(event.source, TelemetrySource::Etw);
    assert_eq!(event.priority, EventPriority::Medium);
    assert_eq!(event.action, TelemetryAction::ProcessStarted);
    assert_eq!(event.confidence_hint, 100);
    assert_eq!(process.process_id, 4242);
    assert_eq!(process.parent_process_id, Some(1000));
    assert_eq!(
        process.image_path.unwrap().as_str(),
        r"C:\Windows\System32\cmd.exe"
    );
    assert_eq!(
        process.command_line.unwrap().as_str(),
        "cmd.exe /c whoami"
    );
}

#[test]
fn process_exit_record_normalizes_to_low_priority_exit_event() {
    let record = EtwProcessRecord::new(
        EtwProcessEventKind::Exit,
        Timestamp::parse_rfc3339("2026-06-27T09:01:00Z").unwrap(),
        4242,
    );

    let event = normalize_process_record(record);

    assert_eq!(event.source, TelemetrySource::Etw);
    assert_eq!(event.priority, EventPriority::Low);
    assert_eq!(event.action, TelemetryAction::ProcessExited);
    assert_eq!(event.process.unwrap().process_id, 4242);
}
