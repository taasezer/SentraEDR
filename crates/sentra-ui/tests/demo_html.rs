use sentra_ui::{
    ActionReviewCard, DashboardState, IpcTelemetryHealth, LiveTelemetryCounters,
    LiveTelemetrySnapshot, render_dashboard_html,
};
use shared_models::{
    Alert, EventPriority, Finding, HealthStatus, RemediationAction, RemediationMode, RiskLevel,
    Signal, Timestamp,
};

#[test]
fn render_dashboard_html_includes_demo_sections_and_metrics() {
    let mut dashboard = DashboardState::from_alerts(
        vec![
            alert_at(RiskLevel::Critical, 95, "2026-06-28T10:01:00Z"),
            alert_at(RiskLevel::High, 81, "2026-06-28T10:02:00Z"),
        ],
        ts("2026-06-28T10:00:00Z"),
    );
    dashboard.apply_live_telemetry(live_snapshot("2026-06-28T10:00:00Z"));
    dashboard.add_pending_action(ActionReviewCard::new(
        "approval-required remediation",
        RemediationMode::ApprovalRequired,
        vec![
            RemediationAction::SuspendProcess,
            RemediationAction::QuarantineFile,
        ],
        ts("2026-06-28T10:03:00Z"),
    ));

    let html = render_dashboard_html(&dashboard);

    assert!(html.contains("<!doctype html>"));
    assert!(html.contains("SentraEDR Dashboard"));
    assert!(html.contains("Events Received"));
    assert!(html.contains("Normalized"));
    assert!(html.contains(">10<"));
    assert!(html.contains("Behavioral Signals"));
    assert!(html.contains(">10<"));
    assert!(html.contains("IPC Frames Accepted"));
    assert!(html.contains(">8<"));
    assert!(html.contains("Risk Summary"));
    assert!(html.contains("Critical Alerts"));
    assert!(html.contains("Pending Actions"));
    assert!(html.contains("Event Timeline"));
    assert!(html.contains("Telemetry update: 10 normalized events"));
    assert!(html.contains("Generated"));
}

#[test]
fn render_dashboard_html_escapes_dynamic_text() {
    let mut dashboard = DashboardState::from_alerts(Vec::new(), ts("2026-06-28T10:00:00Z"));
    dashboard.add_pending_action(ActionReviewCard::new(
        "<script>alert(1)</script>",
        RemediationMode::ApprovalRequired,
        vec![RemediationAction::SuspendProcess],
        ts("2026-06-28T10:03:00Z"),
    ));

    let html = render_dashboard_html(&dashboard);

    assert!(!html.contains("<script>alert(1)</script>"));
    assert!(html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
}

fn alert_at(risk_level: RiskLevel, score: u8, timestamp: &str) -> Alert {
    let mut finding = Finding::new(ts(timestamp), risk_level, score);
    finding.signals.push(Signal {
        name: "synthetic_signal".to_string(),
        description: "synthetic signal".to_string(),
        supporting_event_ids: Vec::new(),
    });
    finding.mitre_techniques.push("T1059.001".to_string());

    Alert {
        alert_id: Default::default(),
        finding,
        recommended_action: "review alert".to_string(),
        remediation_eligible: true,
    }
}

fn live_snapshot(observed_at: &str) -> LiveTelemetrySnapshot {
    LiveTelemetrySnapshot {
        observed_at: ts(observed_at),
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
            detection_alerts: 2,
        },
        ipc: IpcTelemetryHealth {
            enabled: true,
            dispatcher_capacity: 256,
            frames_accepted: 8,
            failed_frames: 1,
        },
    }
}

fn ts(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}
