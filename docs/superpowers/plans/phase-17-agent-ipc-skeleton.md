# Phase 17: Agent IPC Service Skeleton, In-Memory Only Plan

## Goals
Implement the IPC service lifecycle within `sentra-agent`, integrating the `shared-ipc` pipeline for in-memory operation and adding corresponding configuration.

## Implementation Steps

### 1. Configuration Updates
- [ ] Add `IpcConfig` struct to `crates/sentra-agent/src/config.rs`.
- [ ] Add `ipc: IpcConfig` field to `AgentConfig`.
- [ ] Update `AgentConfig::default()` and `AgentConfig::validate()`.
- [ ] Add tests to `crates/sentra-agent/tests/config.rs` to verify IPC config loading.

### 2. IPC Service Implementation
- [ ] Create `crates/sentra-agent/src/ipc.rs`.
- [ ] Implement `IpcService` struct:
    - Wrap `IpcPipeline`.
    - Implement `new(config: &IpcConfig)`.
    - Implement `process_raw_bytes(&mut self, chunk: &[u8])`.
    - Implement `get_stats() -> IpcPipelineStats`.
- [ ] Register `mod ipc` in `crates/sentra-agent/src/lib.rs` (or `main.rs` depending on agent structure).

### 3. Agent Integration & Synthetic Dry-Run
- [ ] Update `sentra-agent`'s main logic to initialize `IpcService`.
- [ la ] Create a synthetic IPC dry-run test in `crates/sentra-agent/tests/ipc_dry_run.rs`.
- [ ] The dry-run should:
    - Use `encode_frame` to create a valid IPC frame.
    - Feed it in chunks to `IpcService`.
    - Verify the message arrives in the dispatcher's queue.
    - Verify `IpcPipelineStats` are updated.

### 4. Refinement and Verification
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Run `cargo test --workspace`.
- [ ] Run architecture validation tools.

## Finalization
- [ ] Update `ARCHITECTURE.md`.
- [ ] Update `IPC_DESIGN.md`.
- [ ] Update `PERFORMANCE_NOTES.md`.
- [ ] Update `TASKS.md`.
- [ ] Create `PHASE_REPORTS/phase-17.md`.
- [ la ] Create `TEST_RESULTS/phase-17.md`.
- [ ] Mark checkboxes in this plan.
