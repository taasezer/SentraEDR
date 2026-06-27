use engine_memory::{MemoryAnalysisReport, MemoryAnalyzer};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, TelemetryAction, TelemetryMetadata, TelemetrySource,
    Timestamp,
};

pub fn run_synthetic_memory_analysis_dry_run() -> MemoryAnalysisReport {
    let mut analyzer = MemoryAnalyzer::default();
    let mut report = analyzer.analyze(memory_event("remote_thread_created"));
    report
        .signals
        .extend(analyzer.analyze(memory_event("executable_private_memory")).signals);
    let final_report = analyzer.analyze(memory_event("protection_changed"));
    report.signals.extend(final_report.signals);
    report.stats = final_report.stats;
    report
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
        Timestamp::parse_rfc3339("2026-06-27T09:09:00Z").unwrap(),
    );
    event.metadata = metadata;
    event
}
