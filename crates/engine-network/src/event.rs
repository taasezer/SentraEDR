use shared_models::{NormalizedTelemetryEvent, TelemetryAction, TelemetryEventId, Timestamp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkDirection {
    Outbound,
    Inbound,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkEvent {
    pub remote_ip: String,
    pub remote_port: u16,
    pub protocol: String,
    pub direction: NetworkDirection,
    pub domain: Option<String>,
    pub process_id: Option<u32>,
    pub supporting_event_id: TelemetryEventId,
    pub observed_at: Timestamp,
}

impl NetworkEvent {
    pub fn from_telemetry(event: &NormalizedTelemetryEvent) -> Option<Self> {
        if event.action != TelemetryAction::NetworkConnectionObserved {
            return None;
        }

        let remote_ip = event.metadata.get("network.remote_ip")?.to_string();
        let remote_port = event.metadata.get("network.remote_port")?.parse().ok()?;
        let protocol = event
            .metadata
            .get("network.protocol")
            .unwrap_or("unknown")
            .to_ascii_lowercase();
        let direction = match event
            .metadata
            .get("network.direction")
            .unwrap_or("unknown")
            .to_ascii_lowercase()
            .as_str()
        {
            "outbound" => NetworkDirection::Outbound,
            "inbound" => NetworkDirection::Inbound,
            _ => NetworkDirection::Unknown,
        };
        let domain = event.metadata.get("network.domain").map(str::to_string);
        let process_id = event
            .metadata
            .get("network.process_id")
            .and_then(|value| value.parse().ok());

        Some(Self {
            remote_ip,
            remote_port,
            protocol,
            direction,
            domain,
            process_id,
            supporting_event_id: event.event_id.clone(),
            observed_at: event.timestamp.clone(),
        })
    }

    pub fn destination_key(&self) -> String {
        self.domain
            .clone()
            .unwrap_or_else(|| self.remote_ip.clone())
            .to_ascii_lowercase()
    }
}
