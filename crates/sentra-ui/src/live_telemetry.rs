use shared_models::{DemoTelemetrySnapshot, EventPriority, HealthStatus, Timestamp};

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

impl LiveTelemetrySnapshot {
    /// Converts a [`DemoTelemetrySnapshot`] from shared-models into the
    /// UI projection model.  The `highest_priority` is derived from
    /// the detection alert count: Critical when alerts > 0, Low otherwise.
    pub fn from_demo_snapshot(snapshot: &DemoTelemetrySnapshot) -> Self {
        let highest_priority = if snapshot.detection_alerts > 0 {
            EventPriority::High
        } else {
            EventPriority::Low
        };

        Self {
            observed_at: snapshot.generated_at.clone(),
            agent_status: snapshot.agent_status,
            highest_priority,
            counters: LiveTelemetryCounters {
                received: snapshot.etw_received,
                normalized: snapshot.etw_normalized,
                dropped: snapshot.etw_dropped,
                process_signals: snapshot.process_signals,
                persistence_signals: snapshot.persistence_signals,
                network_signals: snapshot.network_signals,
                memory_signals: snapshot.memory_signals,
                detection_alerts: snapshot.detection_alerts,
            },
            ipc: IpcTelemetryHealth {
                enabled: snapshot.ipc_dispatcher_capacity > 0,
                dispatcher_capacity: snapshot.ipc_dispatcher_capacity,
                frames_accepted: snapshot.ipc_frames_accepted,
                failed_frames: snapshot.ipc_frames_failed,
            },
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_demo_snapshot() -> DemoTelemetrySnapshot {
        let mut snapshot = DemoTelemetrySnapshot::empty(
            Timestamp::parse_rfc3339("2026-06-28T10:00:00Z").unwrap(),
            HealthStatus::Healthy,
        );
        snapshot.etw_received = 128;
        snapshot.etw_normalized = 124;
        snapshot.etw_dropped = 4;
        snapshot.process_signals = 7;
        snapshot.persistence_signals = 4;
        snapshot.network_signals = 6;
        snapshot.memory_signals = 3;
        snapshot.detection_alerts = 2;
        snapshot.detection_findings = 3;
        snapshot.ipc_frames_accepted = 42;
        snapshot.ipc_frames_failed = 1;
        snapshot.ipc_dispatcher_capacity = 256;
        snapshot
    }

    #[test]
    fn from_demo_snapshot_maps_etw_counters() {
        let demo = sample_demo_snapshot();
        let live = LiveTelemetrySnapshot::from_demo_snapshot(&demo);
        assert_eq!(live.counters.received, 128);
        assert_eq!(live.counters.normalized, 124);
        assert_eq!(live.counters.dropped, 4);
    }

    #[test]
    fn from_demo_snapshot_maps_behavioral_signals() {
        let demo = sample_demo_snapshot();
        let live = LiveTelemetrySnapshot::from_demo_snapshot(&demo);
        assert_eq!(live.counters.process_signals, 7);
        assert_eq!(live.counters.persistence_signals, 4);
        assert_eq!(live.counters.network_signals, 6);
        assert_eq!(live.counters.memory_signals, 3);
    }

    #[test]
    fn from_demo_snapshot_maps_ipc() {
        let demo = sample_demo_snapshot();
        let live = LiveTelemetrySnapshot::from_demo_snapshot(&demo);
        assert!(live.ipc.enabled);
        assert_eq!(live.ipc.dispatcher_capacity, 256);
        assert_eq!(live.ipc.frames_accepted, 42);
        assert_eq!(live.ipc.failed_frames, 1);
    }

    #[test]
    fn from_demo_snapshot_derives_high_priority_when_alerts_exist() {
        let demo = sample_demo_snapshot();
        let live = LiveTelemetrySnapshot::from_demo_snapshot(&demo);
        assert_eq!(live.highest_priority, EventPriority::High);
    }

    #[test]
    fn from_demo_snapshot_derives_low_priority_when_no_alerts() {
        let mut demo = sample_demo_snapshot();
        demo.detection_alerts = 0;
        let live = LiveTelemetrySnapshot::from_demo_snapshot(&demo);
        assert_eq!(live.highest_priority, EventPriority::Low);
    }

    #[test]
    fn from_demo_snapshot_panel_sums_behavioral_signals() {
        let demo = sample_demo_snapshot();
        let live = LiveTelemetrySnapshot::from_demo_snapshot(&demo);
        let panel = LiveTelemetryPanel::from_snapshot(live);
        assert_eq!(panel.behavioral_signals, 7 + 4 + 6 + 3);
    }
}
