use sentra_ui::{
    ActionReviewCard, DashboardState, IpcTelemetryHealth, LiveTelemetryCounters,
    LiveTelemetrySnapshot, TimelineKind,
};
use shared_models::{
    Alert, EventPriority, Finding, HealthStatus, RemediationAction, RemediationMode, RiskLevel,
    Signal, Timestamp,
};

#[test]
fn dashboard_summarizes_alert_risk_counts() {
    let dashboard = DashboardState::from_alerts(
        vec![
            alert(RiskLevel::Critical, 95, true),
            alert(RiskLevel::High, 80, true),
            alert(RiskLevel::Low, 25, false),
        ],
        Timestamp::now(),
    );

    assert_eq!(dashboard.summary.total_alerts, 3);
    assert_eq!(dashboard.summary.critical, 1);
    assert_eq!(dashboard.summary.high, 1);
    assert_eq!(dashboard.summary.low, 1);
    assert_eq!(dashboard.summary.remediation_eligible, 2);
}

#[test]
fn alert_cards_are_sorted_by_score_descending() {
    let dashboard = DashboardState::from_alerts(
        vec![
            alert(RiskLevel::Low, 25, false),
            alert(RiskLevel::Critical, 95, true),
            alert(RiskLevel::High, 80, true),
        ],
        Timestamp::now(),
    );

    let scores: Vec<u8> = dashboard.alerts.iter().map(|alert| alert.score).collect();
    assert_eq!(scores, vec![95, 80, 25]);
}

#[test]
fn timeline_contains_alert_entries_in_timestamp_order() {
    let dashboard = DashboardState::from_alerts(
        vec![
            alert_at(RiskLevel::High, 80, "2026-06-27T09:12:00Z"),
            alert_at(RiskLevel::Critical, 95, "2026-06-27T09:10:00Z"),
        ],
        Timestamp::now(),
    );

    assert_eq!(dashboard.timeline.len(), 2);
    assert_eq!(dashboard.timeline[0].kind, TimelineKind::AlertObserved);
    assert_eq!(
        dashboard.timeline[0].timestamp.to_rfc3339(),
        "2026-06-27T09:10:00+00:00"
    );
}

#[test]
fn pending_action_cards_are_added_to_dashboard() {
    let mut dashboard =
        DashboardState::from_alerts(vec![alert(RiskLevel::High, 80, true)], Timestamp::now());
    dashboard.add_pending_action(ActionReviewCard::new(
        "approval-required remediation",
        RemediationMode::ApprovalRequired,
        vec![
            RemediationAction::SuspendProcess,
            RemediationAction::QuarantineFile,
        ],
        ts("2026-06-27T09:13:00Z"),
    ));

    assert_eq!(dashboard.pending_actions.len(), 1);
    assert_eq!(dashboard.summary.pending_actions, 1);
    assert_eq!(
        dashboard.timeline.last().unwrap().kind,
        TimelineKind::ActionQueued
    );
}

#[test]
fn live_telemetry_updates_dashboard_panel_without_changing_alert_summary() {
    let mut dashboard =
        DashboardState::from_alerts(vec![alert(RiskLevel::High, 80, true)], Timestamp::now());

    dashboard.apply_live_telemetry(live_snapshot("2026-06-28T10:00:00Z"));

    assert_eq!(dashboard.summary.total_alerts, 1);
    assert_eq!(dashboard.summary.high, 1);
    assert_eq!(dashboard.telemetry.total_received, 12);
    assert_eq!(dashboard.telemetry.normalized_events, 10);
    assert_eq!(dashboard.telemetry.behavioral_signals, 10);
    assert_eq!(dashboard.telemetry.ipc_frames_accepted, 8);
}

#[test]
fn live_telemetry_update_is_added_to_sorted_timeline() {
    let mut dashboard = DashboardState::from_alerts(
        vec![alert_at(RiskLevel::High, 80, "2026-06-28T10:01:00Z")],
        Timestamp::now(),
    );

    dashboard.apply_live_telemetry(live_snapshot("2026-06-28T10:00:00Z"));

    assert_eq!(dashboard.timeline.len(), 2);
    assert_eq!(dashboard.timeline[0].kind, TimelineKind::TelemetryUpdated);
    assert_eq!(dashboard.timeline[1].kind, TimelineKind::AlertObserved);
    assert_eq!(
        dashboard.timeline[0].timestamp.to_rfc3339(),
        "2026-06-28T10:00:00+00:00"
    );
}

fn alert(risk_level: RiskLevel, score: u8, remediation_eligible: bool) -> Alert {
    alert_at_eligible(
        risk_level,
        score,
        remediation_eligible,
        "2026-06-27T09:11:00Z",
    )
}

fn alert_at(risk_level: RiskLevel, score: u8, timestamp: &str) -> Alert {
    alert_at_eligible(risk_level, score, true, timestamp)
}

fn alert_at_eligible(
    risk_level: RiskLevel,
    score: u8,
    remediation_eligible: bool,
    timestamp: &str,
) -> Alert {
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
        remediation_eligible,
    }
}

fn ts(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
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
            detection_alerts: 1,
        },
        ipc: IpcTelemetryHealth {
            enabled: true,
            dispatcher_capacity: 256,
            frames_accepted: 8,
            failed_frames: 1,
        },
    }
}
