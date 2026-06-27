use sentra_ui::{
    IpcTelemetryHealth, LiveTelemetryCounters, LiveTelemetryPanel, LiveTelemetrySnapshot,
};
use shared_models::{EventPriority, HealthStatus, Timestamp};

#[test]
fn live_telemetry_snapshot_projects_demo_counters() {
    let snapshot = LiveTelemetrySnapshot {
        observed_at: ts("2026-06-28T10:00:00Z"),
        agent_status: HealthStatus::Healthy,
        highest_priority: EventPriority::High,
        counters: LiveTelemetryCounters {
            received: 12,
            normalized: 10,
            dropped: 2,
            process_signals: 3,
            persistence_signals: 2,
            network_signals: 4,
            memory_signals: 1,
            detection_alerts: 1,
        },
        ipc: IpcTelemetryHealth {
            enabled: true,
            dispatcher_capacity: 256,
            frames_accepted: 8,
            failed_frames: 1,
        },
    };

    let panel = LiveTelemetryPanel::from_snapshot(snapshot);

    assert_eq!(panel.agent_status, HealthStatus::Healthy);
    assert_eq!(panel.highest_priority, EventPriority::High);
    assert_eq!(panel.total_received, 12);
    assert_eq!(panel.normalized_events, 10);
    assert_eq!(panel.dropped_events, 2);
    assert_eq!(panel.behavioral_signals, 10);
    assert_eq!(panel.detection_alerts, 1);
    assert!(panel.ipc_enabled);
    assert_eq!(panel.ipc_dispatcher_capacity, 256);
    assert_eq!(panel.ipc_frames_accepted, 8);
    assert_eq!(panel.ipc_failed_frames, 1);
    assert_eq!(panel.last_updated.to_rfc3339(), "2026-06-28T10:00:00+00:00");
}

fn ts(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}
