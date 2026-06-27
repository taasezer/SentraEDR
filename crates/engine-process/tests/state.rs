use engine_process::{ProcessLifecycleStatus, ProcessStateTable, ProcessStateUpdate};
use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity,
    TelemetryAction, TelemetrySource, Timestamp,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}

fn process_event(
    action: TelemetryAction,
    process_id: u32,
    parent_process_id: Option<u32>,
    image_path: &str,
    command_line: &str,
    observed_at: &str,
) -> NormalizedTelemetryEvent {
    let mut process = ProcessIdentity::new(process_id)
        .with_image_path(ImagePath::new(image_path))
        .with_command_line(CommandLine::new(command_line));
    process.parent_process_id = parent_process_id;

    NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::Medium,
        action,
        timestamp(observed_at),
    )
    .with_process(process)
    .with_confidence_hint(100)
}

#[test]
fn process_start_inserts_running_snapshot() {
    let mut table = ProcessStateTable::default();
    let event = process_event(
        TelemetryAction::ProcessStarted,
        4242,
        Some(1000),
        r"C:\Windows\System32\cmd.exe",
        "cmd.exe /c whoami",
        "2026-06-27T09:00:00Z",
    );

    let update = table.apply_event(&event);

    assert!(matches!(update, ProcessStateUpdate::Started(_)));
    let snapshot = table.get(4242).unwrap();
    assert_eq!(snapshot.process.process_id, 4242);
    assert_eq!(snapshot.process.parent_process_id, Some(1000));
    assert_eq!(snapshot.status, ProcessLifecycleStatus::Running);
    assert_eq!(
        snapshot.first_observed,
        timestamp("2026-06-27T09:00:00Z")
    );
    assert_eq!(
        snapshot.last_observed,
        timestamp("2026-06-27T09:00:00Z")
    );
    assert_eq!(table.len(), 1);
}

#[test]
fn process_exit_marks_existing_process_as_exited() {
    let mut table = ProcessStateTable::default();
    let start = process_event(
        TelemetryAction::ProcessStarted,
        4242,
        Some(1000),
        r"C:\Windows\System32\cmd.exe",
        "cmd.exe",
        "2026-06-27T09:00:00Z",
    );
    let exit = process_event(
        TelemetryAction::ProcessExited,
        4242,
        Some(1000),
        r"C:\Windows\System32\cmd.exe",
        "cmd.exe",
        "2026-06-27T09:01:00Z",
    );

    table.apply_event(&start);
    let update = table.apply_event(&exit);

    assert!(matches!(update, ProcessStateUpdate::Exited(_)));
    let snapshot = table.get(4242).unwrap();
    assert_eq!(snapshot.status, ProcessLifecycleStatus::Exited);
    assert_eq!(
        snapshot.first_observed,
        timestamp("2026-06-27T09:00:00Z")
    );
    assert_eq!(
        snapshot.last_observed,
        timestamp("2026-06-27T09:01:00Z")
    );
    assert_eq!(table.len(), 1);
}

#[test]
fn irrelevant_telemetry_is_ignored_without_state_change() {
    let mut table = ProcessStateTable::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::InternalHealth,
        EventPriority::Low,
        TelemetryAction::ComponentHealthChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );

    let update = table.apply_event(&event);

    assert_eq!(update, ProcessStateUpdate::Ignored);
    assert_eq!(table.len(), 0);
}
