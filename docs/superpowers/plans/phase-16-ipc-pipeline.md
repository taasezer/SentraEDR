# Phase 16: IPC Stream Assembler + Frame Intake Pipeline Composition Plan

## Goals
Implement the `IpcPipeline` in `shared-ipc` that composes the assembler, intake, and dispatcher into a single unit with integrated statistics.

## Implementation Steps

### 1. Define Statistics and Pipeline Structures
- [x] Create `IpcPipelineStats` struct in `crates/shared-ipc/src/lib.rs` (or a new `pipeline.rs`).
- [x] Create `IpcPipeline` struct in `crates/shared-ipc/src/pipeline.rs`.
- [x] Implement `IpcPipeline::new(config: IpcDispatcherConfig)`.

### 2. Implement `process_bytes` Logic
- [x] Implement `IpcPipeline::process_bytes(&mut self, chunk: &[u8]) -> Result<(), IpcError>`.
- [x] Integrate `IpcStreamAssembler::push_bytes`.
- [x] Integrate `IpcFrameIntake::accept_frame`.
- [x] Implement statistics tracking as specified in the design doc.

### 3. TDD: Write Failing Tests
- [x] Create `crates/shared-ipc/tests/pipeline.rs`.
- [x] Test case: `test_pipeline_happy_path` (single frame, multiple chunks).
- [x] Test case: `test_pipeline_fragmented_frames` (multiple frames, overlapping chunks).
- [x] Test case: `test_pipeline_malformed_frame` (decode failure).
- [x] Test case: `test_pipeline_buffer_overflow` (stream rejection).
- [x] Test case: `test_pipeline_dispatch_failure` (queue full).

### 4. Implementation & Fixes
- [x] Implement the logic to make tests green.
- [x] Verify that stats are correctly incremented.

### 5. Refinement and Verification
- [x] Run `cargo fmt --all -- --check`.
- [x] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [x] Run `cargo test --workspace`.
- [x] Run architecture validation tools.

## Finalization
- [x] Update `ARCHITECTURE.md`.
- [x] Update `IPC_DESIGN.md`.
- [x] Update `PERFORMANCE_NOTES.md`.
- [x] Update `TASKS.md`.
- [x] Create `PHASE_REPORTS/phase-16.md`.
- [x] Create `TEST_RESULTS/phase-16.md`.
- [x] Mark checkboxes in this plan.
