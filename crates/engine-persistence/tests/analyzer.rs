use engine_persistence::{PersistenceAnalyzer, PersistenceKind, SignalSeverity};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata, TelemetrySource,
    Timestamp,
};

fn timestamp(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}

fn persistence_event(
    kind: &str,
    path: &str,
    value: &str,
    operation: &str,
) -> NormalizedTelemetryEvent {
    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Medium,
        TelemetryAction::RegistryChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );
    event.metadata = TelemetryMetadata::empty()
        .insert("persistence.kind", kind)
        .insert("persistence.path", path)
        .insert("persistence.value", value)
        .insert("persistence.operation", operation);
    event
}

#[test]
fn run_key_metadata_emits_registry_run_key_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "registry_run_key",
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
        "Updater",
        "set_value",
    ));

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 1);
    assert_eq!(report.stats.ignored, 0);
    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "registry_run_key_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(
        report.signals[0].event.kind,
        PersistenceKind::RegistryRunKey
    );
}

#[test]
fn startup_folder_metadata_emits_startup_folder_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "startup_folder",
        r"C:\Users\alice\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\run.lnk",
        "run.lnk",
        "created",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "startup_folder_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::StartupFolder);
}

#[test]
fn scheduled_task_metadata_emits_scheduled_task_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "scheduled_task",
        r"C:\Windows\System32\Tasks\Updater",
        "Updater",
        "created",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "scheduled_task_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::Medium);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::ScheduledTask);
}

#[test]
fn service_metadata_emits_service_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "service",
        r"HKLM\System\CurrentControlSet\Services\Updater",
        "ImagePath",
        "set_value",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "service_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(report.signals[0].event.kind, PersistenceKind::Service);
}

#[test]
fn wmi_metadata_emits_wmi_signal() {
    let mut analyzer = PersistenceAnalyzer::default();
    let report = analyzer.analyze(persistence_event(
        "wmi",
        r"ROOT\subscription:__EventFilter.Name='Updater'",
        "__EventFilter",
        "created",
    ));

    assert_eq!(report.signals.len(), 1);
    assert_eq!(report.signals[0].name, "wmi_persistence");
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
    assert_eq!(
        report.signals[0].event.kind,
        PersistenceKind::WmiSubscription
    );
}

#[test]
fn irrelevant_telemetry_is_counted_as_ignored() {
    let mut analyzer = PersistenceAnalyzer::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::InternalHealth,
        EventPriority::Low,
        TelemetryAction::ComponentHealthChanged,
        timestamp("2026-06-27T09:00:00Z"),
    );

    let report = analyzer.analyze(event);

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 0);
    assert_eq!(report.stats.ignored, 1);
    assert!(report.signals.is_empty());
}
