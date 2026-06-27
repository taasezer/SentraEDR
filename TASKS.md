# SentraEDR Tasks

Date: 2026-06-27

## Phase 0: Research And Planning

Status: In review

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

Status: Deferred to next approved phase

Reason:

- User review of Phase 0 design is required before implementation planning.

Required output:

- Rust workspace.
- Crate boundaries.
- Shared schemas.
- Logging and config foundation.
- Basic CI/build commands.
- Documentation consistency checks.

Integration impact:

- Phase 1 must encode Phase 0 dependency rules into crate dependencies.

## Phase 2: ETW Telemetry Engine

Status: Deferred

Reason:

- Requires Phase 1 workspace and shared event schemas.

## Phase 3: Process Monitoring Engine

Status: Deferred

Reason:

- Requires process telemetry from Phase 2.

## Phase 4: Persistence Engine

Status: Deferred

Reason:

- Requires shared models and observe-only detection plumbing.

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
