use crate::record::{EtwProcessEventKind, EtwProcessRecord};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, ProcessIdentity, TelemetryAction, TelemetryMetadata,
    TelemetrySource,
};

pub fn normalize_process_record(record: EtwProcessRecord) -> NormalizedTelemetryEvent {
    let (priority, action) = match record.event_kind {
        EtwProcessEventKind::Start => (EventPriority::Medium, TelemetryAction::ProcessStarted),
        EtwProcessEventKind::Exit => (EventPriority::Low, TelemetryAction::ProcessExited),
    };

    let process = ProcessIdentity {
        process_id: record.process_id,
        parent_process_id: record.parent_process_id,
        image_path: record.image_path,
        command_line: record.command_line,
        user_sid: None,
    };

    let mut event =
        NormalizedTelemetryEvent::new(TelemetrySource::Etw, priority, action, record.timestamp)
            .with_process(process)
            .with_confidence_hint(record.confidence);
    event.metadata = TelemetryMetadata::empty().insert("engine", "engine-etw");
    event
}
