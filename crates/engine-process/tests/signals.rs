use engine_process::{ProcessAnalyzer, SignalSeverity};
use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity,
    TelemetryAction, TelemetrySource, Timestamp,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}

fn start_event(
    process_id: u32,
    parent_process_id: Option<u32>,
    image_path: &str,
    command_line: &str,
) -> NormalizedTelemetryEvent {
    let mut process = ProcessIdentity::new(process_id)
        .with_image_path(ImagePath::new(image_path))
        .with_command_line(CommandLine::new(command_line));
    process.parent_process_id = parent_process_id;

    NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::Medium,
        TelemetryAction::ProcessStarted,
        timestamp("2026-06-27T09:00:00Z"),
    )
    .with_process(process)
    .with_confidence_hint(100)
}

#[test]
fn office_to_powershell_emits_suspicious_parent_child_signal() {
    let mut analyzer = ProcessAnalyzer::default();

    analyzer.analyze(start_event(
        1000,
        None,
        r"C:\Program Files\Microsoft Office\root\Office16\WINWORD.EXE",
        "WINWORD.EXE report.docx",
    ));
    let report = analyzer.analyze(start_event(
        4242,
        Some(1000),
        r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        "powershell.exe -NoProfile",
    ));

    assert_eq!(report.stats.observed, 2);
    assert_eq!(report.stats.started, 2);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.tracked_processes, 2);
    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "suspicious_parent_child");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(
        report.signals[0]
            .parent
            .as_ref()
            .unwrap()
            .process
            .process_id,
        1000
    );
}

#[test]
fn powershell_encoded_command_emits_signal() {
    let mut analyzer = ProcessAnalyzer::default();

    let report = analyzer.analyze(start_event(
        4242,
        None,
        r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        "powershell.exe -EncodedCommand SQBFAFgA",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "powershell_encoded_command");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
}

#[test]
fn user_writable_execution_path_emits_signal() {
    let mut analyzer = ProcessAnalyzer::default();

    let report = analyzer.analyze(start_event(
        4242,
        None,
        r"C:\Users\alice\AppData\Local\Temp\payload.exe",
        r"C:\Users\alice\AppData\Local\Temp\payload.exe",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "user_writable_execution_path");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
}

#[test]
fn non_process_event_is_counted_as_ignored() {
    let mut analyzer = ProcessAnalyzer::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::InternalHealth,
        EventPriority::Low,
        TelemetryAction::ComponentHealthChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );

    let report = analyzer.analyze(event);

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.started, 0);
    assert_eq!(report.stats.exited, 0);
    assert_eq!(report.stats.ignored, 1);
    assert!(report.signals.is_empty());
}
