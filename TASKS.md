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

Status: Complete pending user review

Completed:

- Added `engine-network` crate.
- Added network metadata parser.
- Added destination history tracking.
- Added observe-only preliminary network signals.
- Added rare external destination matching.
- Added suspicious DNS pattern matching.
- Added beacon interval candidate matching.
- Added high-risk port matching.
- Added IP-literal outbound connection matching.
- Added agent synthetic network analysis dry run.
- Added architecture validation rules for `engine-network`.

Validation:

- Analyzer tests cover public/private destinations, suspicious DNS, high-risk ports, beacon intervals, IP-literal connections, and ignored telemetry.
- Agent dry-run test covers synthetic network analysis and signal counts.
- Final Phase 5 command results are recorded in `TEST_RESULTS/phase-5.md`.

Architectural impact:

- `engine-network` consumes shared telemetry metadata instead of OS networking APIs.
- `engine-network` depends only on `shared-models`.
- Final scoring, alerting, network isolation, remediation, and UI behavior remain deferred.

## Phase 6: Heuristic Detection Engine

Status: Complete pending user review

Completed:

- Added `engine-detection` crate.
- Added detection signal family and severity model.
- Added heuristic severity and family-diversity scoring.
- Added shared `Finding` generation.
- Added observe-only `Alert` generation.
- Added MITRE technique mapping for initial signal names.
- Added telemetry uncertainty marking for low-confidence signals.
- Added agent synthetic detection dry run.
- Added architecture validation rules for `engine-detection`.

Validation:

- Correlation tests cover multi-family high-risk findings, single-signal low-risk findings, and telemetry uncertainty.
- Agent dry-run test covers observe-only high-risk alert generation.
- Final Phase 6 command results are recorded in `TEST_RESULTS/phase-6.md`.

Architectural impact:

- `engine-detection` depends only on `shared-models`.
- `engine-detection` does not import process, persistence, network, ETW, remediation, agent, or UI crates.
- Alerts remain remediation-ineligible in observe-only mode.

## Phase 7: Quarantine And Remediation Engine

Status: Complete pending user review

Completed:

- Added `engine-remediation` crate.
- Added remediation policy model.
- Added deterministic remediation decision statuses.
- Added remediation plan and planned step model.
- Added remediation audit record generation.
- Added rejection gates for observe-only alerts, telemetry uncertainty, disabled policy, and risk below threshold.
- Added approval-required planning for eligible high-risk alerts.
- Added agent synthetic remediation dry run.
- Added architecture validation rules for `engine-remediation`.

Validation:

- Policy tests cover observe-only rejection, telemetry uncertainty rejection, disabled policy rejection, approval-required planning, and allowed-action constraints.
- Agent dry-run test covers one rejected decision and one waiting-for-approval plan.
- Final Phase 7 command results are recorded in `TEST_RESULTS/phase-7.md`.

Architectural impact:

- `engine-remediation` depends only on `shared-models`.
- `engine-remediation` does not import detection, process, persistence, network, ETW, agent, or UI crates.
- Phase 7 creates plans and audit records only; it does not execute remediation.
- Quarantine, process suspension, network isolation, registry rollback, firewall changes, and deletion remain deferred.

## Phase 8: Memory Inspection Engine

Status: Complete pending user review

Completed:

- Added `engine-memory` crate.
- Added shared `TelemetryAction::MemoryEventObserved`.
- Added memory metadata parser.
- Added observe-only preliminary memory signals.
- Added remote thread creation metadata matching.
- Added executable private memory metadata matching.
- Added unsigned module metadata matching.
- Added suspicious section mapping metadata matching.
- Added memory protection escalation metadata matching.
- Added agent synthetic memory analysis dry run.
- Added architecture validation rules for `engine-memory`.

Validation:

- Analyzer tests cover remote thread, executable private memory, unsigned module, section mapping, protection escalation, and ignored telemetry.
- Agent dry-run test covers synthetic memory analysis and signal counts.
- Final Phase 8 command results are recorded in `TEST_RESULTS/phase-8.md`.

Architectural impact:

- `engine-memory` depends only on `shared-models`.
- `engine-memory` does not import process, persistence, network, ETW, detection, remediation, agent, or UI crates.
- Phase 8 is metadata-only and does not read process memory.
- Real memory scanning, process handle access, memory dumps, kernel drivers, injection, remediation, final alerting, and UI behavior remain deferred.

## Phase 9: UI Dashboard

Status: Complete pending user review

Completed:

- Added `sentra-ui` crate.
- Added dashboard state model.
- Added alert card projection from shared alerts.
- Added risk summary aggregation.
- Added timeline entry model and timestamp ordering.
- Added pending remediation action review cards.
- Added architecture validation rules for `sentra-ui`.

Validation:

- Dashboard tests cover risk summary counts, score sorting, timeline ordering, and pending action queue state.
- Final Phase 9 command results are recorded in `TEST_RESULTS/phase-9.md`.

Architectural impact:

- `sentra-ui` depends only on `shared-models`.
- `sentra-ui` does not import agent, IPC, or engine crates.
- Phase 9 prepares UI-ready state only.
- Browser rendering, local IPC streaming, authentication, user approval execution, and live dashboard workflows remain deferred.

## Phase 10: Testing Infrastructure

Status: Complete pending user review

Completed:

- Added `testing-infra` crate.
- Added safe test scenario model.
- Added scenario safety levels.
- Added scenario kinds for telemetry, process, persistence, network, detection, remediation, memory, UI, and unsafe malware execution markers.
- Added synthetic default scenario catalog for phases 2 through 9.
- Added unsafe scenario rejection.
- Added phase coverage matrix.
- Added coverage report with phase, scenario, and MITRE tag counts.
- Added architecture validation rules for `testing-infra`.

Validation:

- Catalog tests cover synthetic-only defaults, unsafe scenario rejection, implemented phase coverage, missing phase reporting, and coverage report aggregation.
- Final Phase 10 command results are recorded in `TEST_RESULTS/phase-10.md`.

Architectural impact:

- `testing-infra` has no dependency on agent, UI, IPC, or engine crates.
- Phase 10 is planning and validation metadata only.
- Live malware, Atomic Red Team execution, VM orchestration, IPC fuzzing, remediation execution, and host mutation remain deferred.

## Phase 11: CI And Quality Gates

Status: Complete pending user review

Completed:

- Added non-destructive quality gate metadata to `testing-infra`.
- Added safety validation that rejects destructive quality gate commands.
- Added a local PowerShell quality gate runner.
- Added a GitHub Actions workflow for pushes to `Omer` and pull requests.
- Mirrored the same gate order across metadata, local runner, and CI.
- Added Phase 11 report and test results.

Validation:

- Quality gate metadata tests cover ordering, command coverage, non-destructive defaults, and destructive command rejection.
- The local runner covers format, clippy, workspace tests, architecture validation, and observe-only agent dry run.
- Final Phase 11 command results are recorded in `TEST_RESULTS/phase-11.md`.

Architectural impact:

- `testing-infra` remains isolated from agent, UI, IPC, and engine crates.
- Quality gates orchestrate existing validation commands without owning detection or remediation behavior.
- GitHub Actions targets the `Omer` branch; `main` is untouched.

Security notes:

- No malware execution, Atomic Red Team execution, VM orchestration, deployment, release signing, remediation execution, or host mutation was added.
- The agent run remains observe-only.

Performance notes:

- Phase 11 adds validation orchestration only.
- No production telemetry load, benchmark claim, persistent store, unbounded channel, or background scheduler was introduced.
