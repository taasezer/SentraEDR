use engine_persistence::{PersistenceAnalysisReport, PersistenceAnalyzer};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata, TelemetrySource,
    Timestamp,
};

pub fn run_synthetic_persistence_analysis_dry_run() -> PersistenceAnalysisReport {
    let mut analyzer = PersistenceAnalyzer::default();

    let first_report = analyzer.analyze(persistence_event(
        "registry_run_key",
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
        "Updater",
        "set_value",
        "2026-06-27T09:02:00Z",
    ));

    let mut final_report = analyzer.analyze(persistence_event(
        "service",
        r"HKLM\System\CurrentControlSet\Services\Updater",
        "ImagePath",
        "set_value",
        "2026-06-27T09:03:00Z",
    ));
    final_report.signals.splice(0..0, first_report.signals);
    final_report
}

fn persistence_event(
    kind: &str,
    path: &str,
    value: &str,
    operation: &str,
    observed_at: &str,
) -> NormalizedTelemetryEvent {
    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Medium,
        TelemetryAction::RegistryChanged,
        Timestamp::parse_rfc3339(observed_at).unwrap(),
    );
    event.metadata = TelemetryMetadata::empty()
        .insert("persistence.kind", kind)
        .insert("persistence.path", path)
        .insert("persistence.value", value)
        .insert("persistence.operation", operation);
    event
}
