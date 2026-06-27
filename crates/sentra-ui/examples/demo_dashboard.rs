use sentra_ui::{ActionReviewCard, DashboardState, LiveTelemetrySnapshot, render_dashboard_html};
use shared_models::{
    Alert, DemoTelemetrySnapshot, Finding, HealthStatus, RemediationAction, RemediationMode,
    RiskLevel, Signal, Timestamp,
};
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let generated_at = ts("2026-06-28T10:00:00Z");

    let mut dashboard = DashboardState::from_alerts(
        vec![
            alert(
                RiskLevel::Critical,
                95,
                "2026-06-28T10:01:00Z",
                "review PowerShell parent-child chain",
                &["encoded_powershell", "rare_external_destination"],
            ),
            alert(
                RiskLevel::High,
                81,
                "2026-06-28T10:02:00Z",
                "review persistence and memory indicators",
                &["registry_run_key", "executable_private_memory"],
            ),
            alert(
                RiskLevel::Medium,
                55,
                "2026-06-28T10:02:30Z",
                "investigate outbound beacon pattern",
                &["beacon_interval_candidate"],
            ),
            alert(
                RiskLevel::Low,
                20,
                "2026-06-28T10:03:00Z",
                "low-risk informational event",
                &["routine_scheduled_task"],
            ),
        ],
        generated_at.clone(),
    );

    // Build a DemoTelemetrySnapshot matching realistic agent dry-run output
    let demo_snapshot = build_synthetic_demo_snapshot(generated_at);
    let live_snapshot = LiveTelemetrySnapshot::from_demo_snapshot(&demo_snapshot);
    dashboard.apply_live_telemetry(live_snapshot);

    dashboard.add_pending_action(ActionReviewCard::new(
        "approval-required containment plan",
        RemediationMode::ApprovalRequired,
        vec![
            RemediationAction::SuspendProcess,
            RemediationAction::QuarantineFile,
            RemediationAction::BackupRegistryValue,
        ],
        ts("2026-06-28T10:03:00Z"),
    ));

    dashboard.add_pending_action(ActionReviewCard::new(
        "network isolation review",
        RemediationMode::ApprovalRequired,
        vec![
            RemediationAction::IsolateNetwork,
            RemediationAction::BackupRegistryValue,
        ],
        ts("2026-06-28T10:04:00Z"),
    ));

    let output_path = demo_output_path();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, render_dashboard_html(&dashboard))?;
    println!("Generated {}", output_path.display());
    Ok(())
}

/// Constructs a synthetic DemoTelemetrySnapshot with values that match
/// what the agent pipeline would produce from its dry-run cycle.
fn build_synthetic_demo_snapshot(generated_at: Timestamp) -> DemoTelemetrySnapshot {
    let mut snapshot = DemoTelemetrySnapshot::empty(generated_at, HealthStatus::Healthy);
    snapshot.etw_received = 256;
    snapshot.etw_normalized = 248;
    snapshot.etw_dropped = 8;
    snapshot.process_signals = 12;
    snapshot.persistence_signals = 6;
    snapshot.network_signals = 9;
    snapshot.memory_signals = 4;
    snapshot.detection_alerts = 4;
    snapshot.detection_findings = 5;
    snapshot.remediation_decisions = 3;
    snapshot.remediation_waiting_approval = 2;
    snapshot.remediation_planned_steps = 5;
    snapshot.ipc_frames_accepted = 64;
    snapshot.ipc_frames_failed = 2;
    snapshot.ipc_dispatcher_capacity = 256;
    snapshot
}

fn demo_output_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("sentra-demo-dashboard.html")
}

fn alert(
    risk_level: RiskLevel,
    score: u8,
    timestamp: &str,
    recommended_action: &str,
    signals: &[&str],
) -> Alert {
    let mut finding = Finding::new(ts(timestamp), risk_level, score);
    for signal in signals {
        finding.signals.push(Signal {
            name: (*signal).to_string(),
            description: format!("Synthetic demo signal: {signal}"),
            supporting_event_ids: Vec::new(),
        });
    }
    finding.mitre_techniques.push("T1059.001".to_string());
    finding.mitre_techniques.push("T1105".to_string());

    Alert {
        alert_id: Default::default(),
        finding,
        recommended_action: recommended_action.to_string(),
        remediation_eligible: true,
    }
}

fn ts(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}
