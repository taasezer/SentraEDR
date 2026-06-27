# Phase 14 IPC Frame Intake Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an in-memory frame intake adapter that decodes complete IPC frames and dispatches validated envelopes.

**Architecture:** `shared-ipc::intake` composes the existing frame codec and dispatcher. It does not read from OS handles, assemble stream buffers, authorize commands, or execute payloads.

**Tech Stack:** Rust 2024, existing `shared-ipc` frame/dispatcher APIs, existing `shared-models`.

---

### Task 1: Add Frame Intake Tests

**Files:**
- Create: `crates/shared-ipc/tests/intake.rs`
- Modify later: `crates/shared-ipc/src/lib.rs`
- Create later: `crates/shared-ipc/src/intake.rs`

- [ ] **Step 1: Write failing tests**

Create tests for:

- encoded alert frames route to the alert queue;
- malformed frames increment decode failure count;
- full route queues increment dispatch failure count;
- remediation request frames are queued as data.

- [ ] **Step 2: Run RED**

Run: `cargo test -p shared-ipc --test intake`

Expected: fail because `IpcFrameIntake` and `IpcFrameIntakeStats` are missing.

### Task 2: Implement Frame Intake

**Files:**
- Modify: `crates/shared-ipc/src/lib.rs`
- Create: `crates/shared-ipc/src/intake.rs`

- [ ] **Step 1: Add intake stats**

Add `IpcFrameIntakeStats` with `accepted`, `decode_failed`, and `dispatch_failed`.

- [ ] **Step 2: Add intake wrapper**

Add `IpcFrameIntake` with `new`, `accept_frame`, `stats`, `dispatcher`, and `dispatcher_mut`.

- [ ] **Step 3: Compose decode and dispatch**

`accept_frame` must call `decode_frame`, count decode failures, call dispatcher, count dispatch failures, and count accepted frames only after dispatch succeeds.

- [ ] **Step 4: Run GREEN**

Run: `cargo test -p shared-ipc --test intake`

Expected: pass.

### Task 3: Documentation And Verification

**Files:**
- Modify: `ARCHITECTURE.md`
- Modify: `IPC_DESIGN.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-14.md`
- Create: `TEST_RESULTS/phase-14.md`

- [ ] **Step 1: Record Phase 14 docs**

Document that Phase 14 adds in-memory complete-frame intake only.

- [ ] **Step 2: Run final verification**

Run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
cargo run -p sentra-agent
powershell -ExecutionPolicy Bypass -File tools\run-quality-gates.ps1
```

Expected: all commands exit 0.
