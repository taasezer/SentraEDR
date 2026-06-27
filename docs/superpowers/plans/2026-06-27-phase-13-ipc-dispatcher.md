# Phase 13 IPC Dispatcher Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an in-memory bounded IPC dispatcher to `shared-ipc`.

**Architecture:** `shared-ipc::dispatcher` validates `IpcEnvelope` values and routes them into bounded per-message-kind queues. It uses existing bounded queue primitives and does not implement Windows named-pipe transport.

**Tech Stack:** Rust 2024, existing `shared-ipc` queue/message APIs, existing `shared-models`, Tokio test runtime where async receive is needed.

---

### Task 1: Add Dispatcher Tests

**Files:**
- Create: `crates/shared-ipc/tests/dispatcher.rs`
- Modify later: `crates/shared-ipc/src/lib.rs`
- Create later: `crates/shared-ipc/src/dispatcher.rs`

- [x] **Step 1: Write failing tests**

Create tests for:

- alert envelopes route to the alert queue;
- remediation request envelopes route to the remediation request queue;
- mismatched message kind and payload is rejected before enqueueing;
- full route queue returns `QueueFull` and records one dropped message;
- zero capacity dispatcher config is rejected.

- [x] **Step 2: Run RED**

Run: `cargo test -p shared-ipc --test dispatcher`

Expected: fail because `IpcDispatcher`, `IpcDispatcherConfig`, and `IpcRouteStats` are missing.

### Task 2: Implement Dispatcher

**Files:**
- Modify: `crates/shared-ipc/src/error.rs`
- Modify: `crates/shared-ipc/src/lib.rs`
- Create: `crates/shared-ipc/src/dispatcher.rs`

- [x] **Step 1: Add dispatcher error**

Add `InvalidDispatcherCapacity { capacity: usize }`.

- [x] **Step 2: Implement config and stats**

Add `IpcDispatcherConfig` with `queue_capacity` and `try_new`. Add `IpcRouteStats` with `accepted`, `rejected`, and `dropped`.

- [x] **Step 3: Implement dispatcher queues**

Add route-specific bounded senders and receivers for every `IpcMessageKind`.

- [x] **Step 4: Implement dispatch**

Validate the envelope, route it by kind, update stats, and return `QueueFull` when a route queue is full.

- [x] **Step 5: Run GREEN**

Run: `cargo test -p shared-ipc --test dispatcher`

Expected: pass.

### Task 3: Documentation And Verification

**Files:**
- Modify: `ARCHITECTURE.md`
- Modify: `IPC_DESIGN.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-13.md`
- Create: `TEST_RESULTS/phase-13.md`

- [x] **Step 1: Record Phase 13 docs**

Document that Phase 13 adds in-memory dispatch only.

- [x] **Step 2: Run final verification**

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
