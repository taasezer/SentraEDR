# Phase 12 IPC Envelope Codec Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add deterministic IPC envelope and length-prefixed frame codec support to `shared-ipc`.

**Architecture:** `shared-ipc` owns IPC-only message envelopes and frame encoding. It depends on `shared-models` for shared schemas, rejects unsupported schema versions and unsafe/malformed frames, and does not implement named-pipe transport.

**Tech Stack:** Rust 2024, `serde`, `serde_json`, existing `shared-models`, existing workspace quality gates.

---

### Task 1: Add IPC Message Tests

**Files:**
- Create: `crates/shared-ipc/tests/message.rs`
- Modify later: `crates/shared-ipc/src/lib.rs`
- Create later: `crates/shared-ipc/src/message.rs`

- [ ] **Step 1: Write failing message tests**

Create tests for:

- default alert envelope has schema version `SchemaVersion::V1`;
- correlation ID can be attached;
- mismatched message kind and payload is rejected;
- unsupported major schema version is rejected.

- [ ] **Step 2: Run RED**

Run: `cargo test -p shared-ipc --test message`

Expected: fail because `IpcEnvelope`, `IpcMessageKind`, `IpcPayload`, and `MessageId` are missing.

### Task 2: Implement IPC Message Model

**Files:**
- Modify: `crates/shared-ipc/Cargo.toml`
- Modify: `crates/shared-ipc/src/error.rs`
- Modify: `crates/shared-ipc/src/lib.rs`
- Create: `crates/shared-ipc/src/message.rs`

- [ ] **Step 1: Add dependencies**

Add `serde`, `serde_json`, and `uuid` from workspace dependencies to `shared-ipc`.

- [ ] **Step 2: Add error variants**

Add:

- `UnsupportedSchemaVersion { major: u16, minor: u16 }`
- `MessageKindPayloadMismatch { kind: String, payload: String }`

- [ ] **Step 3: Implement message model**

Add `MessageId`, `IpcMessageKind`, `IpcPayload`, `TelemetrySummary`, `RemediationStatusUpdate`, `AuditRecord`, and `IpcEnvelope`.

- [ ] **Step 4: Run GREEN**

Run: `cargo test -p shared-ipc --test message`

Expected: pass.

### Task 3: Add Frame Codec Tests

**Files:**
- Create: `crates/shared-ipc/tests/frame.rs`
- Modify later: `crates/shared-ipc/src/lib.rs`
- Create later: `crates/shared-ipc/src/frame.rs`

- [ ] **Step 1: Write failing frame tests**

Create tests for:

- alert envelope round-trip through frame encode/decode;
- frame has 4-byte big-endian length prefix;
- incomplete frame rejection;
- oversized frame rejection.

- [ ] **Step 2: Run RED**

Run: `cargo test -p shared-ipc --test frame`

Expected: fail because frame codec APIs are missing.

### Task 4: Implement Frame Codec

**Files:**
- Modify: `crates/shared-ipc/src/error.rs`
- Modify: `crates/shared-ipc/src/lib.rs`
- Create: `crates/shared-ipc/src/frame.rs`

- [ ] **Step 1: Add frame errors**

Add:

- `FrameTooLarge { length: usize, max: usize }`
- `IncompleteFrame { expected: usize, actual: usize }`
- `Serialization(String)`

- [ ] **Step 2: Implement encode/decode**

Add `MAX_FRAME_PAYLOAD_BYTES`, `encode_frame`, and `decode_frame`.

- [ ] **Step 3: Run GREEN**

Run: `cargo test -p shared-ipc --test frame`

Expected: pass.

### Task 5: Documentation And Verification

**Files:**
- Modify: `ARCHITECTURE.md`
- Modify: `IPC_DESIGN.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-12.md`
- Create: `TEST_RESULTS/phase-12.md`

- [ ] **Step 1: Record Phase 12 docs**

Document that Phase 12 adds in-memory IPC envelope and frame codec only.

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
