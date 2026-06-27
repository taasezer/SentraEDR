use crate::health::HealthStatus;
use crate::time::Timestamp;

/// Structured summary of a complete agent dry-run cycle.
///
/// Lives in `shared-models` so both `sentra-agent` (producer) and
/// `sentra-ui` (consumer) can reference the same schema without
/// creating a direct dependency between them.
///
/// All fields are primitive types — no engine-internal types leak
/// into this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoTelemetrySnapshot {
    /// When this snapshot was generated.
    pub generated_at: Timestamp,
    /// Overall agent health at snapshot time.
    pub agent_status: HealthStatus,

    // -- ETW ingestion counters --
    pub etw_received: u64,
    pub etw_normalized: u64,
    pub etw_dropped: u64,

    // -- Behavioral engine signal counts --
    pub process_signals: u64,
    pub persistence_signals: u64,
    pub network_signals: u64,
    pub memory_signals: u64,

    // -- Detection engine --
    pub detection_alerts: u64,
    pub detection_findings: u64,

    // -- Remediation planner --
    pub remediation_decisions: u64,
    pub remediation_waiting_approval: u64,
    pub remediation_planned_steps: u64,

    // -- IPC pipeline --
    pub ipc_frames_accepted: u64,
    pub ipc_frames_failed: u64,
    pub ipc_dispatcher_capacity: usize,
}

impl DemoTelemetrySnapshot {
    /// Creates a snapshot with all counters zeroed.
    pub fn empty(generated_at: Timestamp, agent_status: HealthStatus) -> Self {
        Self {
            generated_at,
            agent_status,
            etw_received: 0,
            etw_normalized: 0,
            etw_dropped: 0,
            process_signals: 0,
            persistence_signals: 0,
            network_signals: 0,
            memory_signals: 0,
            detection_alerts: 0,
            detection_findings: 0,
            remediation_decisions: 0,
            remediation_waiting_approval: 0,
            remediation_planned_steps: 0,
            ipc_frames_accepted: 0,
            ipc_frames_failed: 0,
            ipc_dispatcher_capacity: 0,
        }
    }

    /// Total behavioral signals across all engines.
    pub fn total_behavioral_signals(&self) -> u64 {
        self.process_signals + self.persistence_signals + self.network_signals + self.memory_signals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> DemoTelemetrySnapshot {
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
        snapshot.remediation_decisions = 2;
        snapshot.remediation_waiting_approval = 1;
        snapshot.remediation_planned_steps = 3;
        snapshot.ipc_frames_accepted = 42;
        snapshot.ipc_frames_failed = 1;
        snapshot.ipc_dispatcher_capacity = 256;
        snapshot
    }

    #[test]
    fn empty_snapshot_has_zero_counters() {
        let snapshot = DemoTelemetrySnapshot::empty(
            Timestamp::parse_rfc3339("2026-06-28T10:00:00Z").unwrap(),
            HealthStatus::Stopped,
        );
        assert_eq!(snapshot.etw_received, 0);
        assert_eq!(snapshot.total_behavioral_signals(), 0);
        assert_eq!(snapshot.agent_status, HealthStatus::Stopped);
    }

    #[test]
    fn total_behavioral_signals_sums_all_engines() {
        let snapshot = sample_snapshot();
        assert_eq!(snapshot.total_behavioral_signals(), 7 + 4 + 6 + 3);
    }

    #[test]
    fn snapshot_is_clone_and_eq() {
        let snapshot = sample_snapshot();
        let cloned = snapshot.clone();
        assert_eq!(snapshot, cloned);
    }
}
