use engine_process::{ProcessAnalysisReport, ProcessAnalyzer};
use shared_models::{
    CommandLine, EventPriority, ImagePath, NormalizedTelemetryEvent, ProcessIdentity,
    TelemetryAction, TelemetrySource, Timestamp,
};

pub fn run_synthetic_process_analysis_dry_run() -> ProcessAnalysisReport {
    let mut analyzer = ProcessAnalyzer::default();

    analyzer.analyze(process_start(
        1000,
        None,
        r"C:\Program Files\Microsoft Office\root\Office16\WINWORD.EXE",
        "WINWORD.EXE report.docx",
        "2026-06-27T09:00:00Z",
    ));

    analyzer.analyze(process_start(
        4242,
        Some(1000),
        r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
        "powershell.exe -EncodedCommand SQBFAFgA",
        "2026-06-27T09:00:05Z",
    ))
}

fn process_start(
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
        TelemetryAction::ProcessStarted,
        Timestamp::parse_rfc3339(observed_at).unwrap(),
    )
    .with_process(process)
    .with_confidence_hint(100)
}
