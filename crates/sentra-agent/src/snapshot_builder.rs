use crate::detection_dry_run::run_synthetic_detection_dry_run;
use crate::dry_run::run_synthetic_etw_dry_run;
use crate::ipc_dry_run::run_synthetic_ipc_dry_run;
use crate::memory_dry_run::run_synthetic_memory_analysis_dry_run;
use crate::network_dry_run::run_synthetic_network_analysis_dry_run;
use crate::persistence_dry_run::run_synthetic_persistence_analysis_dry_run;
use crate::process_dry_run::run_synthetic_process_analysis_dry_run;
use crate::remediation_dry_run::run_synthetic_remediation_dry_run;
use shared_models::{DemoTelemetrySnapshot, HealthStatus, Timestamp};

/// Runs every synthetic dry-run pipeline and collects the results
/// into a single [`DemoTelemetrySnapshot`].
///
/// The snapshot contains only primitive counters — no engine-internal
/// types leak through.  This function is safe to call in any context;
/// all underlying dry-runs use synthetic data only.
pub fn build_demo_snapshot() -> DemoTelemetrySnapshot {
    let etw_report = run_synthetic_etw_dry_run();
    let process_report = run_synthetic_process_analysis_dry_run();
    let persistence_report = run_synthetic_persistence_analysis_dry_run();
    let network_report = run_synthetic_network_analysis_dry_run();
    let memory_report = run_synthetic_memory_analysis_dry_run();
    let detection_report = run_synthetic_detection_dry_run();
    let remediation_report = run_synthetic_remediation_dry_run();
    let ipc_report = run_synthetic_ipc_dry_run().expect("synthetic IPC dry-run should succeed");

    let config = crate::config::AgentConfig::default();

    DemoTelemetrySnapshot {
        generated_at: Timestamp::now(),
        agent_status: HealthStatus::Healthy,

        etw_received: etw_report.stats.received,
        etw_normalized: etw_report.stats.normalized,
        etw_dropped: etw_report.stats.dropped,

        process_signals: process_report.signals.len() as u64,
        persistence_signals: persistence_report.signals.len() as u64,
        network_signals: network_report.signals.len() as u64,
        memory_signals: memory_report.signals.len() as u64,

        detection_alerts: detection_report.alerts.len() as u64,
        detection_findings: detection_report.findings.len() as u64,

        remediation_decisions: remediation_report.decisions_evaluated,
        remediation_waiting_approval: remediation_report.waiting_for_approval,
        remediation_planned_steps: remediation_report.planned_steps as u64,

        ipc_frames_accepted: ipc_report.stats.frames_accepted,
        ipc_frames_failed: ipc_report.stats.intake_decode_failed
            + ipc_report.stats.intake_dispatch_failed,
        ipc_dispatcher_capacity: config.ipc.dispatcher_capacity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared_models::HealthStatus;

    #[test]
    fn build_demo_snapshot_produces_nonzero_etw_counters() {
        let snapshot = build_demo_snapshot();
        assert!(
            snapshot.etw_received > 0,
            "ETW received should be > 0 from synthetic dry-run"
        );
        assert!(
            snapshot.etw_normalized > 0,
            "ETW normalized should be > 0 from synthetic dry-run"
        );
    }

    #[test]
    fn build_demo_snapshot_has_healthy_status() {
        let snapshot = build_demo_snapshot();
        assert_eq!(snapshot.agent_status, HealthStatus::Healthy);
    }

    #[test]
    fn build_demo_snapshot_has_behavioral_signals() {
        let snapshot = build_demo_snapshot();
        assert!(
            snapshot.total_behavioral_signals() > 0,
            "At least one behavioral signal expected from dry-runs"
        );
    }

    #[test]
    fn build_demo_snapshot_has_detection_findings() {
        let snapshot = build_demo_snapshot();
        assert!(
            snapshot.detection_findings > 0,
            "Detection dry-run should produce findings"
        );
    }

    #[test]
    fn build_demo_snapshot_has_ipc_frames() {
        let snapshot = build_demo_snapshot();
        assert!(
            snapshot.ipc_frames_accepted > 0,
            "IPC dry-run should accept at least one frame"
        );
    }

    #[test]
    fn build_demo_snapshot_has_remediation_decisions() {
        let snapshot = build_demo_snapshot();
        assert!(
            snapshot.remediation_decisions > 0,
            "Remediation dry-run should evaluate decisions"
        );
    }
}
