# Phase 17: Agent IPC Service Skeleton, In-Memory Only Plan

## Goals
Implement the IPC service lifecycle within `sentra-agent`, integrating the `shared-ipc` pipeline for in-memory operation and adding corresponding configuration.

## Implementation Steps

### 1. Configuration Updates
- [x] Add `IpcConfig` struct to `crates/sentra-agent/src/config.rs`.
- [x] Add `ipc: IpcConfig` field to `AgentConfig`.
- [x] Update `AgentConfig::default()` and `AgentConfig::validate()`.
- [x] Add tests to `crates/sentra-agent/tests/config.rs` to verify IPC config loading.

### 2. IPC Service Implementation
- [x] Create `crates/sentra-agent/src/ipc.rs`.
- [x] Implement `IpcService` struct:
    - [x] Wrap `IpcPipeline`.
    - [x] Implement `new(config: IpcConfig)`.
    - [x] Implement `process_raw_bytes(&mut self, chunk: &[u8])`.
    - [x] Implement `stats() -> IpcPipelineStats`.
- [x] Register `mod ipc` in `crates/sentra-agent/src/lib.rs` (or `main.rs` depending on agent structure).

### 3. Agent Integration & Synthetic Dry-Run
- [x] Update `sentra-agent`'s main logic to initialize `IpcService`.
- [x] Create a synthetic IPC dry-run test in `crates/sentra-agent/tests/ipc_dry_run.rs`.
- [x] The dry-run should:
    - [x] Use `encode_frame` to create a valid IPC frame.
    - [x] Feed it in chunks to `IpcService`.
    - [x] Verify the message arrives in the dispatcher's queue.
    - [x] Verify `IpcPipelineStats` are updated.

### 4. Refinement and Verification
- [x] Run `cargo fmt --all -- --check`.
- [x] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Run `cargo test --workspace`.
- [x] Run architecture validation tools.

## Finalization
- [x] Update `ARCHITECTURE.md`.
- [x] Update `IPC_DESIGN.md`.
- [x] Update `PERFORMANCE_NOTES.md`.
- [x] Update `TASKS.md`.
- [x] Create `PHASE_REPORTS/phase-17.md`.
- [x] Create `TEST_RESULTS/phase-17.md`.
- [x] Mark checkboxes in this plan.
