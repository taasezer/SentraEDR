use engine_memory::{MemoryAnalyzer, SignalSeverity};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata, TelemetrySource,
    Timestamp,
};

#[test]
fn remote_thread_metadata_emits_high_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(memory_event("remote_thread_created"));

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 1);
    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "remote_thread_creation")
    );
    assert_eq!(report.signals[0].severity, SignalSeverity::High);
}

#[test]
fn executable_private_memory_emits_high_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report =
        analyzer.analyze(memory_event("executable_private_memory").with_confidence_hint(75));

    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "executable_private_memory")
    );
}

#[test]
fn unsigned_module_metadata_emits_medium_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(memory_event("unsigned_module_loaded").with_confidence_hint(80));

    let signal = report
        .signals
        .iter()
        .find(|s| s.name == "unsigned_module_loaded")
        .unwrap();
    assert_eq!(signal.severity, SignalSeverity::Medium);
}

#[test]
fn section_mapping_metadata_emits_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(memory_event("section_mapping"));

    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "suspicious_section_mapping")
    );
}

#[test]
fn protection_change_to_execute_emits_signal() {
    let mut analyzer = MemoryAnalyzer::default();
    let report = analyzer.analyze(memory_event("protection_changed").with_confidence_hint(90));

    assert!(
        report
            .signals
            .iter()
            .any(|s| s.name == "memory_protection_escalation")
    );
}

#[test]
fn non_memory_event_is_ignored() {
    let mut analyzer = MemoryAnalyzer::default();
    let event = NormalizedTelemetryEvent::new(
        TelemetrySource::WindowsEventLog,
        EventPriority::Low,
        TelemetryAction::ProcessStarted,
        ts(),
    );

    let report = analyzer.analyze(event);

    assert_eq!(report.stats.observed, 1);
    assert_eq!(report.stats.handled, 0);
    assert_eq!(report.stats.ignored, 1);
    assert!(report.signals.is_empty());
}

fn memory_event(kind: &str) -> NormalizedTelemetryEvent {
    let metadata = TelemetryMetadata::empty()
        .insert("memory.event_type", kind)
        .insert("memory.source_process_id", "4242")
        .insert("memory.target_process_id", "9001")
        .insert("memory.module_path", r"C:\Users\Public\stage.dll")
        .insert("memory.protection", "PAGE_EXECUTE_READWRITE")
        .insert("memory.region_kind", "private")
        .insert("memory.allocation_size", "4096")
        .insert("memory.thread_start_address", "0x1000");

    let mut event = NormalizedTelemetryEvent::new(
        TelemetrySource::Etw,
        EventPriority::High,
        TelemetryAction::MemoryEventObserved,
        ts(),
    );
    event.metadata = metadata;
    event
}

fn ts() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:09:00Z").unwrap()
}
