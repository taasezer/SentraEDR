# Phase 18 UI Live Telemetry Projection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a demo-ready live telemetry projection layer to `sentra-ui`.

**Architecture:** `sentra-ui` remains a pure UI state/projection crate and still depends only on `shared-models`. The new `live_telemetry` module accepts inert snapshot values and projects them into `DashboardState` without importing `sentra-agent`, `shared-ipc`, or engine crates.

**Tech Stack:** Rust workspace crates, `shared-models`, TDD integration tests, existing PowerShell quality gates.

---

### Task 1: Live Telemetry Projection Types

**Files:**
- Create: `crates/sentra-ui/src/live_telemetry.rs`
- Modify: `crates/sentra-ui/src/lib.rs`
- Test: `crates/sentra-ui/tests/live_telemetry.rs`

- [x] **Step 1: Write the failing projection test**

Create `crates/sentra-ui/tests/live_telemetry.rs` with a test that imports `LiveTelemetryCounters`, `IpcTelemetryHealth`, `LiveTelemetryPanel`, and `LiveTelemetrySnapshot`, constructs a snapshot, and asserts projected totals:

```rust
use sentra_ui::{
    IpcTelemetryHealth, LiveTelemetryCounters, LiveTelemetryPanel, LiveTelemetrySnapshot,
};
use shared_models::{EventPriority, HealthStatus, Timestamp};

#[test]
fn live_telemetry_snapshot_projects_demo_counters() {
    let snapshot = LiveTelemetrySnapshot {
        observed_at: ts("2026-06-28T10:00:00Z"),
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
    };

    let panel = LiveTelemetryPanel::from_snapshot(snapshot);

    assert_eq!(panel.agent_status, HealthStatus::Healthy);
    assert_eq!(panel.highest_priority, EventPriority::High);
    assert_eq!(panel.total_received, 12);
    assert_eq!(panel.normalized_events, 10);
    assert_eq!(panel.dropped_events, 2);
    assert_eq!(panel.behavioral_signals, 10);
    assert_eq!(panel.detection_alerts, 1);
    assert!(panel.ipc_enabled);
    assert_eq!(panel.ipc_frames_accepted, 8);
    assert_eq!(panel.ipc_failed_frames, 1);
    assert_eq!(panel.last_updated.to_rfc3339(), "2026-06-28T10:00:00+00:00");
}

fn ts(value: &str) -> Timestamp {
    Timestamp::parse_rfc3339(value).unwrap()
}
```

- [x] **Step 2: Run the test to verify it fails**

Run: `cargo test -p sentra-ui --test live_telemetry`

Expected: FAIL because `LiveTelemetrySnapshot` and related types are not exported yet.

- [x] **Step 3: Add the minimal implementation**

Create `crates/sentra-ui/src/live_telemetry.rs` with the snapshot, counter, IPC health, and panel structs plus `LiveTelemetryPanel::from_snapshot`.

- [x] **Step 4: Export the module and types**

Modify `crates/sentra-ui/src/lib.rs` to add `pub mod live_telemetry;` and re-export the four new public types.

- [x] **Step 5: Run the test to verify it passes**

Run: `cargo test -p sentra-ui --test live_telemetry`

Expected: PASS.

### Task 2: Dashboard Integration

**Files:**
- Modify: `crates/sentra-ui/src/dashboard.rs`
- Modify: `crates/sentra-ui/src/timeline.rs`
- Test: `crates/sentra-ui/tests/dashboard.rs`

- [x] **Step 1: Write failing dashboard tests**

Add tests that call `DashboardState::apply_live_telemetry(snapshot)` and verify:

- `dashboard.telemetry` is updated.
- `summary` alert counts are unchanged.
- a `TelemetryUpdated` timeline entry is added.
- the combined timeline remains sorted.

- [x] **Step 2: Run the dashboard test to verify it fails**

Run: `cargo test -p sentra-ui --test dashboard`

Expected: FAIL because `apply_live_telemetry`, `telemetry`, and `TimelineKind::TelemetryUpdated` do not exist yet.

- [x] **Step 3: Add `TelemetryUpdated` timeline kind**

Modify `crates/sentra-ui/src/timeline.rs` to include `TelemetryUpdated`.

- [x] **Step 4: Add telemetry state to dashboard**

Modify `DashboardState` to include `pub telemetry: LiveTelemetryPanel` and initialize it with `Default::default()`.

- [x] **Step 5: Add `apply_live_telemetry`**

Implement a method that projects the snapshot into a panel, pushes a `TelemetryUpdated` timeline entry, and sorts the timeline.

- [x] **Step 6: Run dashboard tests**

Run: `cargo test -p sentra-ui --test dashboard`

Expected: PASS.

### Task 3: Documentation And Verification

**Files:**
- Modify: `TASKS.md`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Create: `PHASE_REPORTS/phase-18.md`
- Create: `TEST_RESULTS/phase-18.md`
- Modify: `docs/superpowers/plans/2026-06-28-phase-18-ui-live-telemetry-projection.md`

- [x] **Step 1: Update docs for Phase 18**

Record that Phase 18 adds UI-side live telemetry projection only, with no renderer, transport, command authorization, or remediation execution.

- [x] **Step 2: Run full verification**

Run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
powershell -ExecutionPolicy Bypass -File tools\run-quality-gates.ps1
```

Expected: every command exits 0.

- [x] **Step 3: Record final results**

Create `TEST_RESULTS/phase-18.md` with targeted test and final verification results.

- [x] **Step 4: Commit**

Commit all Phase 18 changes on `Omer` with:

```powershell
git add .
git commit -m "feat: add ui live telemetry projection for phase 18"
```
