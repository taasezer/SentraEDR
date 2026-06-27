use shared_models::{
    Alert, AlertId, RemediationAction, RemediationMode, RemediationStatus, RiskLevel, Timestamp,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationPolicy {
    pub mode: RemediationMode,
    pub minimum_risk_level: RiskLevel,
    pub manual_approval_required: bool,
    pub allowed_actions: Vec<RemediationAction>,
}

impl RemediationPolicy {
    pub fn approval_required() -> Self {
        Self {
            mode: RemediationMode::ApprovalRequired,
            minimum_risk_level: RiskLevel::High,
            manual_approval_required: true,
            allowed_actions: vec![
                RemediationAction::SuspendProcess,
                RemediationAction::IsolateNetwork,
                RemediationAction::QuarantineFile,
            ],
        }
    }

    pub fn disabled() -> Self {
        Self {
            mode: RemediationMode::Disabled,
            minimum_risk_level: RiskLevel::Critical,
            manual_approval_required: true,
            allowed_actions: Vec::new(),
        }
    }

    pub fn with_allowed_actions(mut self, allowed_actions: Vec<RemediationAction>) -> Self {
        self.allowed_actions = allowed_actions;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationDecisionStatus {
    RejectedByPolicy,
    WaitingForApproval,
    Approved,
}

impl From<RemediationDecisionStatus> for RemediationStatus {
    fn from(status: RemediationDecisionStatus) -> Self {
        match status {
            RemediationDecisionStatus::RejectedByPolicy => RemediationStatus::RejectedByPolicy,
            RemediationDecisionStatus::WaitingForApproval => RemediationStatus::WaitingForApproval,
            RemediationDecisionStatus::Approved => RemediationStatus::Approved,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationPlanStepKind {
    SuspendProcess,
    IsolateNetwork,
    QuarantineFile,
    BackupRegistryValue,
}

impl RemediationPlanStepKind {
    fn from_action(action: RemediationAction) -> Option<Self> {
        match action {
            RemediationAction::SuspendProcess => Some(Self::SuspendProcess),
            RemediationAction::IsolateNetwork => Some(Self::IsolateNetwork),
            RemediationAction::QuarantineFile => Some(Self::QuarantineFile),
            RemediationAction::BackupRegistryValue => Some(Self::BackupRegistryValue),
            RemediationAction::RestoreRegistryValue => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationPlanStep {
    pub kind: RemediationPlanStepKind,
    pub action: RemediationAction,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationPlan {
    pub plan_id: Uuid,
    pub alert_id: AlertId,
    pub created_at: Timestamp,
    pub mode: RemediationMode,
    pub steps: Vec<RemediationPlanStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationAuditRecord {
    pub audit_id: Uuid,
    pub alert_id: AlertId,
    pub status: RemediationDecisionStatus,
    pub status_model: RemediationStatus,
    pub mode: RemediationMode,
    pub rationale: String,
    pub recorded_at: Timestamp,
    pub planned_steps: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationDecision {
    pub status: RemediationDecisionStatus,
    pub plan: Option<RemediationPlan>,
    pub audit: RemediationAuditRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationEngine {
    policy: RemediationPolicy,
}

impl RemediationEngine {
    pub fn new(policy: RemediationPolicy) -> Self {
        Self { policy }
    }

    pub fn evaluate(&self, alert: &Alert, evaluated_at: Timestamp) -> RemediationDecision {
        if self.policy.mode == RemediationMode::Disabled {
            return self.reject(alert, evaluated_at, "remediation policy is disabled");
        }

        if !alert.remediation_eligible {
            return self.reject(alert, evaluated_at, "alert is not remediation eligible");
        }

        if alert.finding.telemetry_uncertainty {
            return self.reject(alert, evaluated_at, "alert has telemetry uncertainty");
        }

        if risk_rank(alert.finding.risk_level) < risk_rank(self.policy.minimum_risk_level) {
            return self.reject(alert, evaluated_at, "finding risk is below policy threshold");
        }

        let steps = self.plan_steps();
        if steps.is_empty() {
            return self.reject(alert, evaluated_at, "policy has no allowed remediation actions");
        }

        let plan = RemediationPlan {
            plan_id: Uuid::new_v4(),
            alert_id: alert.alert_id.clone(),
            created_at: evaluated_at.clone(),
            mode: self.policy.mode,
            steps,
        };
        let status = if self.policy.manual_approval_required {
            RemediationDecisionStatus::WaitingForApproval
        } else {
            RemediationDecisionStatus::Approved
        };

        self.decide(
            alert,
            evaluated_at,
            status,
            Some(plan),
            "eligible alert produced a gated remediation plan",
        )
    }

    fn reject(
        &self,
        alert: &Alert,
        evaluated_at: Timestamp,
        rationale: &'static str,
    ) -> RemediationDecision {
        self.decide(
            alert,
            evaluated_at,
            RemediationDecisionStatus::RejectedByPolicy,
            None,
            rationale,
        )
    }

    fn decide(
        &self,
        alert: &Alert,
        evaluated_at: Timestamp,
        status: RemediationDecisionStatus,
        plan: Option<RemediationPlan>,
        rationale: &'static str,
    ) -> RemediationDecision {
        let planned_steps = plan.as_ref().map_or(0, |plan| plan.steps.len());
        RemediationDecision {
            status,
            plan,
            audit: RemediationAuditRecord {
                audit_id: Uuid::new_v4(),
                alert_id: alert.alert_id.clone(),
                status,
                status_model: status.into(),
                mode: self.policy.mode,
                rationale: rationale.to_string(),
                recorded_at: evaluated_at,
                planned_steps,
            },
        }
    }

    fn plan_steps(&self) -> Vec<RemediationPlanStep> {
        self.policy
            .allowed_actions
            .iter()
            .filter_map(|action| {
                let kind = RemediationPlanStepKind::from_action(*action)?;
                Some(RemediationPlanStep {
                    kind,
                    action: *action,
                    description: step_description(kind).to_string(),
                })
            })
            .collect()
    }
}

fn step_description(kind: RemediationPlanStepKind) -> &'static str {
    match kind {
        RemediationPlanStepKind::SuspendProcess => {
            "Plan to suspend the suspicious process after explicit approval"
        }
        RemediationPlanStepKind::IsolateNetwork => {
            "Plan to isolate network access after explicit approval"
        }
        RemediationPlanStepKind::QuarantineFile => {
            "Plan to quarantine the related file after explicit approval"
        }
        RemediationPlanStepKind::BackupRegistryValue => {
            "Plan to back up registry state before any rollback"
        }
    }
}

fn risk_rank(risk_level: RiskLevel) -> u8 {
    match risk_level {
        RiskLevel::Informational => 0,
        RiskLevel::Low => 1,
        RiskLevel::Medium => 2,
        RiskLevel::High => 3,
        RiskLevel::Critical => 4,
    }
}
