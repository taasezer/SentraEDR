use engine_remediation::{
    RemediationDecisionStatus, RemediationEngine, RemediationPolicy, RemediationPlan,
};
use shared_models::{Alert, Finding, RiskLevel, Timestamp};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RemediationDryRunReport {
    pub decisions_evaluated: u64,
    pub rejected_by_policy: u64,
    pub waiting_for_approval: u64,
    pub approved: u64,
    pub planned_steps: usize,
    pub plans: Vec<RemediationPlan>,
}

pub fn run_synthetic_remediation_dry_run() -> RemediationDryRunReport {
    let engine = RemediationEngine::new(RemediationPolicy::approval_required());
    let decisions = vec![
        engine.evaluate(&observe_only_alert(), ts()),
        engine.evaluate(&eligible_high_risk_alert(), ts()),
    ];

    let mut report = RemediationDryRunReport {
        decisions_evaluated: decisions.len() as u64,
        ..Default::default()
    };

    for decision in decisions {
        match decision.status {
            RemediationDecisionStatus::RejectedByPolicy => report.rejected_by_policy += 1,
            RemediationDecisionStatus::WaitingForApproval => report.waiting_for_approval += 1,
            RemediationDecisionStatus::Approved => report.approved += 1,
        }
        if let Some(plan) = decision.plan {
            report.planned_steps += plan.steps.len();
            report.plans.push(plan);
        }
    }

    report
}

fn observe_only_alert() -> Alert {
    Alert::observe_only(finding(RiskLevel::High, 80), "observe only review")
}

fn eligible_high_risk_alert() -> Alert {
    Alert {
        alert_id: Default::default(),
        finding: finding(RiskLevel::High, 85),
        recommended_action: "queue approval-required remediation plan".to_string(),
        remediation_eligible: true,
    }
}

fn finding(risk_level: RiskLevel, score: u8) -> Finding {
    let mut finding = Finding::new(ts(), risk_level, score);
    finding.mitre_techniques.push("T1071".to_string());
    finding
}

fn ts() -> Timestamp {
    Timestamp::parse_rfc3339("2026-06-27T09:08:00Z").unwrap()
}
