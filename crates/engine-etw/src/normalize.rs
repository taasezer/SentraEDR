use crate::record::{EtwNetworkEventKind, EtwNetworkRecord, EtwProcessEventKind, EtwProcessRecord, EtwRecord};
use shared_models::{
    EventPriority, NormalizedTelemetryEvent, ProcessIdentity, TelemetryAction, TelemetryMetadata,
    TelemetrySource,
};

pub fn normalize_etw_record(record: EtwRecord) -> NormalizedTelemetryEvent {
    match record {
        EtwRecord::Process(p) => normalize_process_record(p),
        EtwRecord::Network(n) => normalize_network_record(n),
    }
}

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

pub fn normalize_network_record(record: EtwNetworkRecord) -> NormalizedTelemetryEvent {
    let (priority, action) = match record.event_kind {
        EtwNetworkEventKind::TcpConnect => (EventPriority::High, TelemetryAction::NetworkConnectionObserved),
        EtwNetworkEventKind::TcpAccept => (EventPriority::Medium, TelemetryAction::NetworkConnectionObserved),
        EtwNetworkEventKind::TcpDisconnect => (EventPriority::Low, TelemetryAction::NetworkConnectionObserved),
        EtwNetworkEventKind::UdpSend | EtwNetworkEventKind::UdpReceive => {
            (EventPriority::Low, TelemetryAction::NetworkConnectionObserved)
        }
    };

    let process = ProcessIdentity {
        process_id: record.process_id,
        parent_process_id: None,
        image_path: None,
        command_line: None,
        user_sid: None,
    };

    let mut event =
        NormalizedTelemetryEvent::new(TelemetrySource::Etw, priority, action, record.timestamp)
            .with_process(process)
            .with_confidence_hint(100);
            
    let mut metadata = TelemetryMetadata::empty().insert("engine", "engine-etw");
    metadata = metadata.insert("network.remote_ip", record.remote_ip);
    metadata = metadata.insert("network.remote_port", record.remote_port.to_string());
    metadata = metadata.insert("network.local_port", record.local_port.to_string());
    
    let direction = match record.event_kind {
        EtwNetworkEventKind::TcpConnect => "outbound",
        EtwNetworkEventKind::TcpAccept => "inbound",
        EtwNetworkEventKind::UdpSend => "outbound",
        EtwNetworkEventKind::UdpReceive => "inbound",
        _ => "unknown",
    };
    metadata = metadata.insert("network.direction", direction);
    metadata = metadata.insert("network.protocol", "tcp"); // Simplification for MVP

    event.metadata = metadata;
    event
}
