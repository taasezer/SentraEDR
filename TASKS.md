# SentraEDR Tasks

Date: 2026-06-27

## Phase 0: Research And Planning

Status: Approved by user on 2026-06-27

Completed:

- Defined initial architecture boundaries.
- Defined planned crate ownership.
- Defined telemetry pipeline contract.
- Defined security model and trust boundaries.
- Defined threat model and safe test strategy.
- Defined detection model and remediation safety gates.
- Defined memory and IPC design.
- Defined performance targets and metrics.

Validation:

- Architecture consistency: designed, not implemented.
- Security validation: design risks identified.
- Memory validation: target and controls defined, no measurements yet.
- IPC compatibility: message categories and schema rules defined.
- Telemetry consistency: normalized event contract defined.

Architectural impact:

- Later phases must keep engines isolated through `shared-models` and `shared-ipc`.
- UI must remain outside core detection logic.
- Remediation must remain gated and auditable.

Security notes:

- Observe-only mode is the default until remediation controls are implemented.
- Controlled test sources only: Atomic Red Team, EICAR where relevant, and benign local simulators.

Performance notes:

- All production telemetry paths must use bounded queues.
- UI receives summaries and alerts instead of raw high-volume telemetry by default.

## Phase 1: Workspace And Architecture Initialization

Status: Complete pending user review

Completed:

- Installed Rust toolchain for this workstation.
- Initialized Rust workspace root.
- Added `shared-models` schema crate.
- Added `shared-ipc` bounded queue primitive.
- Added `sentra-agent` config and logging foundation.
- Added architecture dependency validation script.
- Added Phase 1 report and test results.

Validation:

- `cargo fmt --all -- --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `powershell -ExecutionPolicy Bypass -File tools/validate-architecture.ps1` passed.
- `cargo run -p sentra-agent` initialized the agent foundation in observe-only mode.

Architectural impact:

- Phase 0 dependency direction is represented by crate layout.
- `shared-models` remains dependency-light.
- `shared-ipc` depends on `shared-models` only.
- `sentra-agent` wires foundations without owning detection logic.

Security notes:

- Agent defaults to observe-only mode.
- No remediation executor exists.
- No ETW, named-pipe server, Windows service, or UI exists yet.

Performance notes:

- Bounded queue primitive records depth and dropped events.
- No runtime memory benchmark exists yet.

Compatibility notes:

- The workstation lacked MSVC `link.exe`.
- Visual Studio Build Tools installation through `winget` exited with code 1602.
- Phase 1 validation used `stable-x86_64-pc-windows-gnu` through `rust-toolchain.toml`.

## Phase 2: ETW Telemetry Engine

Status: Complete pending user review

Completed:

- Added `engine-etw` crate.
- Added process start and process exit record models.
- Added process lifecycle normalization into shared telemetry events.
- Added deterministic synthetic ETW source.
- Added bounded queue ingestion runner.
- Added ingestion stats and component health reporting.
- Added queue pressure handling with dropped-event accounting.
- Added agent synthetic ETW dry run in observe-only mode.
- Added architecture validation rules for `engine-etw`.

Validation:

- Normalizer tests cover process start and process exit events.
- Ingestion tests cover bounded delivery and queue pressure.
- Agent dry-run test covers two normalized synthetic events.
- Final Phase 2 command results are recorded in `TEST_RESULTS/phase-2.md`.

Architectural impact:

- `engine-etw` depends only on shared crates.
- Real Windows ETW session and callback code remains deferred.
- No detection scoring or remediation is performed by telemetry ingestion.

Performance notes:

- Bounded queue pressure is observable through queue health.
- Dropped telemetry is counted and degrades component health.
- Real ETW burst and memory benchmarks are not claimed in this phase.

## Phase 3: Process Monitoring Engine

Status: Complete pending user review

Completed:

- Added `engine-process` crate.
- Added process lifecycle state table.
- Added start and exit event handling from normalized telemetry.
- Added observe-only preliminary process signals.
- Added suspicious parent-child process chain matching.
- Added PowerShell encoded command flag matching.
- Added user-writable execution path matching.
- Added agent synthetic process analysis dry run.
- Added architecture validation rules for `engine-process`.

Validation:

- State tests cover process start, process exit, and irrelevant telemetry.
- Signal tests cover suspicious parent-child, encoded PowerShell, user-writable execution paths, and ignored non-process events.
- Agent dry-run test covers synthetic process analysis and signal counts.
- Final Phase 3 command results are recorded in `TEST_RESULTS/phase-3.md`.

Architectural impact:

- `engine-process` consumes shared telemetry schemas instead of ETW-specific records.
- `engine-process` depends only on `shared-models`.
- Final scoring, alerting, remediation, and UI behavior remain deferred.

Performance notes:

- State is in-memory and deterministic for Phase 3 test data.
- Matching uses conservative string checks.
- No real telemetry throughput or memory-retention benchmark is claimed yet.

## Phase 4: Persistence Engine

Status: Complete pending user review

Completed:

- Added `engine-persistence` crate.
- Added persistence metadata parser.
- Added persistence analysis report and stats.
- Added observe-only preliminary persistence signals.
- Added registry Run key persistence matching.
- Added startup folder persistence matching.
- Added scheduled task persistence matching.
- Added service persistence matching.
- Added WMI persistence matching.
- Added agent synthetic persistence analysis dry run.
- Added architecture validation rules for `engine-persistence`.

Validation:

- Analyzer tests cover Run key, startup folder, scheduled task, service, WMI, and ignored telemetry.
- Agent dry-run test covers synthetic persistence analysis and signal counts.
- Final Phase 4 command results are recorded in `TEST_RESULTS/phase-4.md`.

Architectural impact:

- `engine-persistence` consumes shared telemetry metadata instead of Windows API types.
- `engine-persistence` depends only on `shared-models`.
- Final scoring, alerting, rollback, remediation, and UI behavior remain deferred.

Performance notes:

- Matching uses conservative string checks.
- No regex engine, persistent store, Windows API access, or unbounded channel is introduced.
- No real registry or Windows Event Log throughput benchmark is claimed yet.

## Phase 5: Network Engine

Status: Deferred

Reason:

- Requires core runtime and queue design from earlier phases.

## Phase 6: Heuristic Detection Engine

Status: Deferred

Reason:

- Requires multiple signal families for meaningful correlation.

## Phase 7: Quarantine And Remediation Engine

Status: Deferred

Reason:

- Requires validated detection confidence, audit, and rollback contracts.

## Phase 8: Memory Inspection Engine

Status: Deferred

Reason:

- Requires stable core telemetry and careful safety review.

## Phase 9: UI Dashboard

Status: Deferred

Reason:

- Requires stable agent IPC and alert schemas.

## Phase 10: Testing Infrastructure

Status: Deferred

Reason:

- Requires executable agent components and VM test environment.
