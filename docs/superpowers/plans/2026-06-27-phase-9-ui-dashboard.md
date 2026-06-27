# Phase 9 UI Dashboard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a UI dashboard foundation crate that converts shared alerts and remediation review items into deterministic UI-ready state.

**Architecture:** `sentra-ui` depends only on `shared-models`. It owns presentation state and does not render a live web UI, connect IPC, import engines, approve remediation, or execute actions.

**Tech Stack:** Rust workspace crate, `shared-models`, deterministic unit tests, existing architecture validation script.

---

### Task 1: Add Dashboard State Tests

**Files:**
- Create: `crates/sentra-ui/tests/dashboard.rs`
- Modify later: `Cargo.toml`
- Create later: `crates/sentra-ui/Cargo.toml`
- Create later: `crates/sentra-ui/src/lib.rs`
- Create later: `crates/sentra-ui/src/alert_card.rs`
- Create later: `crates/sentra-ui/src/action_queue.rs`
- Create later: `crates/sentra-ui/src/timeline.rs`
- Create later: `crates/sentra-ui/src/dashboard.rs`

- [ ] **Step 1: Write failing tests**

```rust
use sentra_ui::{ActionReviewCard, DashboardState, TimelineKind};
use shared_models::{Alert, Finding, RemediationAction, RemediationMode, RiskLevel, Signal, Timestamp};

#[test]
fn dashboard_summarizes_alert_risk_counts() {
    let dashboard = DashboardState::from_alerts(vec![
        alert(RiskLevel::Critical, 95, true),
        alert(RiskLevel::High, 80, true),
        alert(RiskLevel::Low, 25, false),
    ]);

    assert_eq!(dashboard.summary.total_alerts, 3);
    assert_eq!(dashboard.summary.critical, 1);
    assert_eq!(dashboard.summary.high, 1);
    assert_eq!(dashboard.summary.low, 1);
    assert_eq!(dashboard.summary.remediation_eligible, 2);
}

#[test]
fn alert_cards_are_sorted_by_score_descending() {
    let dashboard = DashboardState::from_alerts(vec![
        alert(RiskLevel::Low, 25, false),
        alert(RiskLevel::Critical, 95, true),
        alert(RiskLevel::High, 80, true),
    ]);

    let scores: Vec<u8> = dashboard.alerts.iter().map(|alert| alert.score).collect();
    assert_eq!(scores, vec![95, 80, 25]);
}

#[test]
fn timeline_contains_alert_entries_in_timestamp_order() {
    let dashboard = DashboardState::from_alerts(vec![
        alert_at(RiskLevel::High, 80, "2026-06-27T09:12:00Z"),
        alert_at(RiskLevel::Critical, 95, "2026-06-27T09:10:00Z"),
    ]);

    assert_eq!(dashboard.timeline.len(), 2);
    assert_eq!(dashboard.timeline[0].kind, TimelineKind::AlertObserved);
    assert_eq!(dashboard.timeline[0].timestamp.to_rfc3339(), "2026-06-27T09:10:00+00:00");
}

#[test]
fn pending_action_cards_are_added_to_dashboard() {
    let mut dashboard = DashboardState::from_alerts(vec![alert(RiskLevel::High, 80, true)]);
    dashboard.add_pending_action(ActionReviewCard::new(
        "approval-required remediation",
        RemediationMode::ApprovalRequired,
        vec![RemediationAction::SuspendProcess, RemediationAction::QuarantineFile],
        ts("2026-06-27T09:13:00Z"),
    ));

    assert_eq!(dashboard.pending_actions.len(), 1);
    assert_eq!(dashboard.summary.pending_actions, 1);
    assert_eq!(dashboard.timeline.last().unwrap().kind, TimelineKind::ActionQueued);
}

fn alert(risk_level: RiskLevel, score: u8, remediation_eligible: bool) -> Alert {
    alert_at_eligible(risk_level, score, remediation_eligible, "2026-06-27T09:11:00Z")
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
```

- [ ] **Step 2: Run RED**

Run: `cargo test -p sentra-ui --test dashboard`

Expected: FAIL because `sentra-ui` is not a workspace package.

### Task 2: Implement UI Dashboard State

**Files:**
- Modify: `Cargo.toml`
- Create: `crates/sentra-ui/Cargo.toml`
- Create: `crates/sentra-ui/src/lib.rs`
- Create: `crates/sentra-ui/src/alert_card.rs`
- Create: `crates/sentra-ui/src/action_queue.rs`
- Create: `crates/sentra-ui/src/timeline.rs`
- Create: `crates/sentra-ui/src/dashboard.rs`

- [ ] **Step 1: Add workspace crate**

Add `"crates/sentra-ui"` to workspace members.

- [ ] **Step 2: Add crate dependency**

```toml
[dependencies]
shared-models = { path = "../shared-models" }
```

- [ ] **Step 3: Implement dashboard state**

Implement `AlertCard`, `ActionReviewCard`, `TimelineEntry`, `TimelineKind`, `RiskSummary`, and `DashboardState`.

- [ ] **Step 4: Run GREEN**

Run: `cargo test -p sentra-ui --test dashboard`

Expected: PASS with 4 tests.

### Task 3: Architecture And Documentation

**Files:**
- Modify: `tools/validate-architecture.ps1`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-9.md`
- Create: `TEST_RESULTS/phase-9.md`

- [ ] **Step 1: Add architecture validation rules**

Add rules that prevent `sentra-ui` from depending on agent, engine crates, or `shared-ipc`.

- [ ] **Step 2: Update phase documentation**

Record Phase 9 as dashboard state foundation only. Document that no browser renderer, IPC client, approval execution, or live UI is implemented.

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

