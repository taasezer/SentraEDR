# Phase 7 Remediation Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a safe remediation planning engine that gates alerts through policy and audit before any future executor can act.

**Architecture:** `engine-remediation` depends only on `shared-models`. It creates remediation decisions, plans, and audit records, but it performs no operating system action.

**Tech Stack:** Rust workspace crate, `shared-models`, deterministic unit tests, existing architecture validation script.

---

### Task 1: Add Remediation Policy Tests

**Files:**
- Create: `crates/engine-remediation/tests/policy.rs`
- Create later: `crates/engine-remediation/Cargo.toml`
- Create later: `crates/engine-remediation/src/lib.rs`

- [ ] **Step 1: Write failing policy tests**

```rust
use engine_remediation::{
    RemediationDecisionStatus, RemediationEngine, RemediationPolicy, RemediationPlanStepKind,
};
use shared_models::{Alert, Finding, RemediationAction, RemediationMode, RiskLevel, Timestamp};

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
    assert!(plan.steps.iter().any(|step| step.kind == RemediationPlanStepKind::SuspendProcess));
    assert!(plan.steps.iter().any(|step| step.kind == RemediationPlanStepKind::IsolateNetwork));
    assert!(plan.steps.iter().any(|step| step.kind == RemediationPlanStepKind::QuarantineFile));
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
    assert_eq!(plan.steps[0].kind, RemediationPlanStepKind::BackupRegistryValue);
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
```

- [ ] **Step 2: Run test to verify RED**

Run: `cargo test -p engine-remediation --test policy`

Expected: FAIL because `engine-remediation` does not exist.

### Task 2: Implement Remediation Engine

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/engine-remediation/Cargo.toml`
- Create: `crates/engine-remediation/src/lib.rs`

- [ ] **Step 1: Add the workspace crate**

```toml
"crates/engine-remediation",
```

- [ ] **Step 2: Add crate dependencies**

```toml
[dependencies]
shared-models = { path = "../shared-models" }
uuid = { workspace = true }
```

- [ ] **Step 3: Implement public API**

Implement `RemediationPolicy`, `RemediationEngine`, `RemediationDecision`, `RemediationPlan`, `RemediationPlanStep`, `RemediationPlanStepKind`, and `RemediationAuditRecord`.

- [ ] **Step 4: Run GREEN test**

Run: `cargo test -p engine-remediation --test policy`

Expected: PASS with 5 tests.

### Task 3: Add Agent Dry Run

**Files:**
- Modify: `crates/sentra-agent/Cargo.toml`
- Create: `crates/sentra-agent/src/remediation_dry_run.rs`
- Modify: `crates/sentra-agent/src/lib.rs`
- Modify: `crates/sentra-agent/src/main.rs`
- Create: `crates/sentra-agent/tests/remediation_dry_run.rs`

- [ ] **Step 1: Write failing agent dry-run test**

```rust
#[test]
fn synthetic_remediation_dry_run_reports_rejection_and_approval_queue() {
    let report = sentra_agent::remediation_dry_run::run_synthetic_remediation_dry_run();

    assert_eq!(report.decisions_evaluated, 2);
    assert_eq!(report.rejected_by_policy, 1);
    assert_eq!(report.waiting_for_approval, 1);
    assert_eq!(report.planned_steps, 3);
}
```

- [ ] **Step 2: Run test to verify RED**

Run: `cargo test -p sentra-agent --test remediation_dry_run`

Expected: FAIL because the module is missing.

- [ ] **Step 3: Implement dry run**

Create one observe-only alert and one eligible high-risk alert. Evaluate both through approval-required policy and return aggregate counts.

- [ ] **Step 4: Run GREEN test**

Run: `cargo test -p sentra-agent --test remediation_dry_run`

Expected: PASS with 1 test.

### Task 4: Architecture And Documentation

**Files:**
- Modify: `tools/validate-architecture.ps1`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-7.md`
- Create: `TEST_RESULTS/phase-7.md`

- [ ] **Step 1: Add architecture validation rules**

Add rules that prevent `engine-remediation` from depending on agent, UI, or peer engine crates.

- [ ] **Step 2: Update phase documentation**

Record that Phase 7 adds safe remediation planning only. Document that no OS remediation executor is implemented.

- [ ] **Step 3: Run final verification**

Run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
cargo run -p sentra-agent
```

Expected: all commands exit 0.

