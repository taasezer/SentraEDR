use engine_etw::{
    EtwIngestionReport, EtwIngestor, EtwProcessEventKind, EtwProcessRecord, SyntheticEtwSource,
};
use shared_ipc::bounded_channel;
use shared_models::{NormalizedTelemetryEvent, Timestamp};

pub fn run_synthetic_etw_dry_run() -> EtwIngestionReport {
    let records = vec![
        EtwProcessRecord::new(
            EtwProcessEventKind::Start,
            Timestamp::parse_rfc3339("2026-06-27T09:00:00Z").unwrap(),
            4242,
        )
        .with_parent_process_id(1000)
        .with_image_path(r"C:\Windows\System32\cmd.exe")
        .with_command_line("cmd.exe /c whoami"),
        EtwProcessRecord::new(
            EtwProcessEventKind::Exit,
            Timestamp::parse_rfc3339("2026-06-27T09:01:00Z").unwrap(),
            4242,
        ),
    ];
    let source = SyntheticEtwSource::from_records(records);
    let (sender, _receiver) =
        bounded_channel::<NormalizedTelemetryEvent>("etw-process-dry-run", 16);

    EtwIngestor::new(source, sender).drain()
}
