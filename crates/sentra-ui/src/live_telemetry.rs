use shared_models::{EventPriority, HealthStatus, Timestamp};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LiveTelemetryCounters {
    pub received: u64,
    pub normalized: u64,
    pub dropped: u64,
    pub process_signals: u64,
    pub persistence_signals: u64,
    pub network_signals: u64,
    pub memory_signals: u64,
    pub detection_alerts: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IpcTelemetryHealth {
    pub enabled: bool,
    pub dispatcher_capacity: usize,
    pub frames_accepted: u64,
    pub failed_frames: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveTelemetrySnapshot {
    pub observed_at: Timestamp,
    pub agent_status: HealthStatus,
    pub highest_priority: EventPriority,
    pub counters: LiveTelemetryCounters,
    pub ipc: IpcTelemetryHealth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveTelemetryPanel {
    pub agent_status: HealthStatus,
    pub highest_priority: EventPriority,
    pub total_received: u64,
    pub normalized_events: u64,
    pub dropped_events: u64,
    pub behavioral_signals: u64,
    pub detection_alerts: u64,
    pub ipc_enabled: bool,
    pub ipc_dispatcher_capacity: usize,
    pub ipc_frames_accepted: u64,
    pub ipc_failed_frames: u64,
    pub last_updated: Timestamp,
}

impl LiveTelemetryPanel {
    pub fn from_snapshot(snapshot: LiveTelemetrySnapshot) -> Self {
        Self {
            agent_status: snapshot.agent_status,
            highest_priority: snapshot.highest_priority,
            total_received: snapshot.counters.received,
            normalized_events: snapshot.counters.normalized,
            dropped_events: snapshot.counters.dropped,
            behavioral_signals: snapshot.counters.process_signals
                + snapshot.counters.persistence_signals
                + snapshot.counters.network_signals
                + snapshot.counters.memory_signals,
            detection_alerts: snapshot.counters.detection_alerts,
            ipc_enabled: snapshot.ipc.enabled,
            ipc_dispatcher_capacity: snapshot.ipc.dispatcher_capacity,
            ipc_frames_accepted: snapshot.ipc.frames_accepted,
            ipc_failed_frames: snapshot.ipc.failed_frames,
            last_updated: snapshot.observed_at,
        }
    }
}

impl Default for LiveTelemetryPanel {
    fn default() -> Self {
        Self {
            agent_status: HealthStatus::Stopped,
            highest_priority: EventPriority::Low,
            total_received: 0,
            normalized_events: 0,
            dropped_events: 0,
            behavioral_signals: 0,
            detection_alerts: 0,
            ipc_enabled: false,
            ipc_dispatcher_capacity: 0,
            ipc_frames_accepted: 0,
            ipc_failed_frames: 0,
            last_updated: Timestamp::now(),
        }
    }
}
