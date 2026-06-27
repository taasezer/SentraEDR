use sentra_ui::{
    ActionReviewCard, DashboardState, IpcTelemetryHealth, LiveTelemetryCounters,
    LiveTelemetrySnapshot, render_dashboard_html,
};
use shared_models::{
    Alert, EventPriority, Finding, HealthStatus, RemediationAction, RemediationMode, RiskLevel,
    Signal, Timestamp,
};
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let mut dashboard = DashboardState::from_alerts(vec![
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
    ]);

    dashboard.apply_live_telemetry(LiveTelemetrySnapshot {
        observed_at: ts("2026-06-28T10:00:00Z"),
        agent_status: HealthStatus::Healthy,
        highest_priority: EventPriority::High,
        counters: LiveTelemetryCounters {
            received: 128,
            normalized: 124,
            dropped: 4,
            process_signals: 7,
            persistence_signals: 4,
            network_signals: 6,
            memory_signals: 3,
            detection_alerts: 2,
        },
        ipc: IpcTelemetryHealth {
            enabled: true,
            dispatcher_capacity: 256,
            frames_accepted: 42,
            failed_frames: 1,
        },
    });

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

    let output_path = demo_output_path();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, render_dashboard_html(&dashboard))?;
    println!("Generated {}", output_path.display());
    Ok(())
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
