use engine_remediation::{
    RemediationDecisionStatus, RemediationEngine, RemediationPlanStepKind, RemediationPolicy,
};
use shared_models::{Alert, Finding, RemediationAction, RiskLevel, Timestamp};

#[test]
fn observe_only_alert_is_rejected_by_policy() {
    let mut finding = Finding::new(ts(), RiskLevel::High, 80);
    finding.mitre_techniques.push("T1059.001".to_string());
    let alert = Alert::observe_only(finding, "review only");
    let engine = RemediationEngine::new(RemediationPolicy::approval_required());

    let decision = engine.evaluate(&alert, ts());

    assert_eq!(decision.status, RemediationDecisionStatus::RejectedByPolicy);
    assert!(decision.plan.is_none());
    assert!(decision.audit.rationale.contains("not remediation eligible"));
}

#[test]
fn telemetry_uncertainty_rejects_remediation() {
    let mut finding = Finding::new(ts(), RiskLevel::Critical, 95);
    finding.telemetry_uncertainty = true;
    let alert = eligible_alert(finding);
    let engine = RemediationEngine::new(RemediationPolicy::approval_required());

    let decision = engine.evaluate(&alert, ts());

    assert_eq!(decision.status, RemediationDecisionStatus::RejectedByPolicy);
    assert!(decision.plan.is_none());
    assert!(decision.audit.rationale.contains("telemetry uncertainty"));
}

#[test]
fn high_risk_eligible_alert_creates_approval_required_plan() {
    let mut finding = Finding::new(ts(), RiskLevel::High, 85);
    finding.mitre_techniques.push("T1071".to_string());
    let alert = eligible_alert(finding);
    let engine = RemediationEngine::new(RemediationPolicy::approval_required());

    let decision = engine.evaluate(&alert, ts());

    assert_eq!(decision.status, RemediationDecisionStatus::WaitingForApproval);
    let plan = decision.plan.expect("plan should be present");
    assert_eq!(plan.steps.len(), 3);
    assert!(
        plan.steps
            .iter()
            .any(|step| step.kind == RemediationPlanStepKind::SuspendProcess)
    );
    assert!(
        plan.steps
            .iter()
            .any(|step| step.kind == RemediationPlanStepKind::IsolateNetwork)
    );
    assert!(
        plan.steps
            .iter()
            .any(|step| step.kind == RemediationPlanStepKind::QuarantineFile)
    );
    assert_eq!(decision.audit.planned_steps, 3);
}

#[test]
fn disabled_policy_rejects_all_remediation() {
    let alert = eligible_alert(Finding::new(ts(), RiskLevel::Critical, 95));
    let engine = RemediationEngine::new(RemediationPolicy::disabled());

    let decision = engine.evaluate(&alert, ts());

    assert_eq!(decision.status, RemediationDecisionStatus::RejectedByPolicy);
    assert!(decision.plan.is_none());
    assert!(decision.audit.rationale.contains("disabled"));
}

#[test]
fn policy_actions_constrain_generated_plan() {
    let alert = eligible_alert(Finding::new(ts(), RiskLevel::Critical, 95));
    let policy = RemediationPolicy::approval_required()
        .with_allowed_actions(vec![RemediationAction::BackupRegistryValue]);
    let engine = RemediationEngine::new(policy);

    let decision = engine.evaluate(&alert, ts());

    let plan = decision.plan.expect("plan should be present");
    assert_eq!(plan.steps.len(), 1);
    assert_eq!(
        plan.steps[0].kind,
        RemediationPlanStepKind::BackupRegistryValue
    );
}

fn eligible_alert(finding: Finding) -> Alert {
    Alert {
        alert_id: Default::default(),
        finding,
        recommended_action: "approval required".to_string(),
        remediation_eligible: true,
    }
}

fn ts() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:08:00Z").unwrap()
}
