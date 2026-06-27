# Phase 15 IPC Stream Assembler Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a bounded byte-stream assembler that emits complete IPC frames from arbitrary chunks.

**Architecture:** `shared-ipc::stream` parses only the 4-byte length prefix and buffers incomplete bytes. It does not decode JSON, dispatch envelopes, open pipes, or execute payloads.

**Tech Stack:** Rust 2024, existing `shared-ipc` frame constants and errors, existing test helpers.

---

### Task 1: Add Stream Assembler Tests

**Files:**
- Create: `crates/shared-ipc/tests/stream.rs`
- Modify later: `crates/shared-ipc/src/lib.rs`
- Create later: `crates/shared-ipc/src/stream.rs`

- [ ] **Step 1: Write failing tests**

Create tests for:

- a frame split across two chunks is emitted only after the second chunk;
- two complete frames in one chunk are both emitted;
- an oversized length prefix is rejected before payload buffering;
- partial bytes remain buffered and appear in stats.

- [ ] **Step 2: Run RED**

Run: `cargo test -p shared-ipc --test stream`

Expected: fail because `IpcStreamAssembler` and `IpcStreamAssemblerStats` are missing.

### Task 2: Implement Stream Assembler

**Files:**
- Modify: `crates/shared-ipc/src/error.rs`
- Modify: `crates/shared-ipc/src/frame.rs`
- Modify: `crates/shared-ipc/src/lib.rs`
- Create: `crates/shared-ipc/src/stream.rs`

- [ ] **Step 1: Export frame prefix size**

Expose `FRAME_PREFIX_BYTES` from `frame.rs` for assembler use.

- [ ] **Step 2: Add stream buffer error**

Add `StreamBufferTooLarge { length: usize, max: usize }`.

- [ ] **Step 3: Add assembler stats**

Add `IpcStreamAssemblerStats` with `frames_completed`, `bytes_buffered`, and `rejected`.

- [ ] **Step 4: Implement `push_bytes`**

Append bytes into a bounded buffer, parse complete frames, remove emitted bytes, reject oversized prefixes, and update stats.

- [ ] **Step 5: Run GREEN**

Run: `cargo test -p shared-ipc --test stream`

Expected: pass.

### Task 3: Documentation And Verification

**Files:**
- Modify: `ARCHITECTURE.md`
- Modify: `IPC_DESIGN.md`
- Modify: `PERFORMANCE_NOTES.md`
- Modify: `TASKS.md`
- Create: `PHASE_REPORTS/phase-15.md`
- Create: `TEST_RESULTS/phase-15.md`

- [ ] **Step 1: Record Phase 15 docs**

Document that Phase 15 adds bounded in-memory stream assembly only.

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
