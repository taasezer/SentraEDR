# Phase 3 Process Monitoring Design

Date: 2026-06-27
Status: Approved for implementation planning
Branch target: `Omer`

## Goal

Phase 3 adds a lightweight `engine-process` crate that consumes normalized process telemetry, maintains process lifecycle state, and emits observe-only process behavior signals.

This phase does not implement full detection correlation, alerting, remediation, UI streaming, named-pipe IPC, real ETW sessions, signer reputation, or memory inspection.

## Context

Phase 2 added `engine-etw`, which normalizes synthetic process start and process exit records into `NormalizedTelemetryEvent`.

Phase 3 sits between telemetry ingestion and the future detection engine:

```text
engine-etw
  -> NormalizedTelemetryEvent
  -> engine-process
  -> ProcessAnalysisReport
  -> future engine-detection
```

`engine-process` is stateful because process behavior often depends on lineage and recent lifecycle context. It is still observe-only and cannot remediate.

## Crate Boundary

Create:

- `crates/engine-process`

Allowed dependencies:

- `shared-models`
- `shared-ipc` only if a specific bounded delivery task is approved for Phase 3
- `thiserror` if an error type is required

Forbidden dependencies:

- `sentra-agent`
- `sentra-ui`
- peer engines such as `engine-etw`, `engine-network`, `engine-persistence`, `engine-detection`, and `engine-remediation`

The engine consumes shared telemetry schemas, not ETW-specific record types. This keeps process analysis decoupled from the telemetry provider.

## Inputs

Input type:

- `NormalizedTelemetryEvent`

Relevant event actions:

- `TelemetryAction::ProcessStarted`
- `TelemetryAction::ProcessExited`

Events without process identity are ignored and counted as ignored telemetry.

Non-process events are ignored and counted as ignored telemetry.

## Outputs

Primary output:

- `ProcessAnalysisReport`

The report contains:

- number of events observed;
- number of process starts handled;
- number of process exits handled;
- number of ignored events;
- current tracked process count;
- emitted process signals;
- optional component health.

Signal output:

- `ProcessSignal`

Signals are preliminary observations, not final findings. The future detection phase can correlate them with network, persistence, PowerShell, and reputation signals.

## Process State

`ProcessStateTable` stores a snapshot per PID:

- process ID;
- parent process ID;
- image path;
- command line;
- first observed timestamp;
- last observed timestamp;
- lifecycle status.

Lifecycle status values:

- `Running`
- `Exited`

Exit events mark a process as exited instead of deleting it immediately. This preserves short-lived process lineage for future analysis.

The initial implementation can keep all observed state in memory. Retention limits are documented as future work because Phase 3 test data is deterministic and small.

## Initial Signals

Phase 3 implements three deterministic process behavior signals.

### Suspicious Parent-Child

Emit `suspicious_parent_child` when a process start has a suspicious parent/child pair.

Initial pairs:

- Office parent launching PowerShell:
  - `winword.exe -> powershell.exe`
  - `excel.exe -> powershell.exe`
  - `powerpnt.exe -> powershell.exe`
- Browser parent launching script interpreter:
  - `chrome.exe -> powershell.exe`
  - `msedge.exe -> powershell.exe`
  - `firefox.exe -> powershell.exe`
- Archive tool parent launching script interpreter:
  - `winrar.exe -> powershell.exe`
  - `7z.exe -> powershell.exe`

The signal includes:

- signal name;
- description;
- process snapshot;
- optional parent snapshot;
- supporting telemetry event ID.

### PowerShell Encoded Command

Emit `powershell_encoded_command` when a PowerShell process command line contains an encoded command flag.

Initial case-insensitive matches:

- `-enc`
- `/enc`
- `-encodedcommand`
- `/encodedcommand`

This is a signal only. It is not an automatic high-severity finding because legitimate administration tools can use encoded commands.

### User-Writable Execution Path

Emit `user_writable_execution_path` when a process image path appears to run from a user-writable location.

Initial case-insensitive path fragments:

- `\users\`
- `\appdata\local\temp\`
- `\appdata\roaming\`
- `\downloads\`
- `\temp\`

The first implementation uses conservative string matching and must not claim filesystem ACL verification.

## Scoring

Phase 3 does not perform final risk scoring.

Each `ProcessSignal` can include a local severity hint:

- `Low`
- `Medium`
- `High`

These hints help future detection work but do not create alerts or remediation eligibility.

Suggested initial hints:

- suspicious parent-child: `High`
- PowerShell encoded command: `Medium`
- user-writable execution path: `Medium`

## Error Handling

Malformed or irrelevant telemetry is not fatal.

The engine should:

- increment ignored counts for unsupported events;
- keep processing after missing optional process fields;
- avoid panics in normal analysis paths;
- return a report even when no signals are emitted.

## Security Boundaries

`engine-process` must not:

- kill or suspend processes;
- quarantine files;
- change firewall rules;
- edit registry keys;
- create alerts directly;
- import ETW-specific types;
- import detection, remediation, network, persistence, memory, agent, or UI crates.

Signals are evidence for future correlation, not final verdicts.

## Performance Constraints

The implementation should stay allocation-light:

- process snapshots are small owned values;
- signal matching uses normalized lowercase strings only when needed;
- no regex dependency is required for Phase 3;
- no unbounded channel is introduced.

The state table is in-memory and deterministic for Phase 3. Retention, eviction, and memory pressure policies are future work.

## Testing Strategy

Tests must be TDD-first.

Required tests:

- process start inserts a running process snapshot;
- process exit marks an existing process as exited;
- Office-to-PowerShell emits `suspicious_parent_child`;
- PowerShell encoded command emits `powershell_encoded_command`;
- executable under a user-writable path emits `user_writable_execution_path`;
- irrelevant telemetry is ignored without panic;
- architecture validation rejects forbidden `engine-process` dependencies.

Agent integration should have a synthetic dry-run test after the core engine is implemented.

## Documentation Updates

Phase 3 implementation must update:

- `ARCHITECTURE.md`
- `PERFORMANCE_NOTES.md`
- `TASKS.md`
- `PHASE_REPORTS/phase-3.md`
- `TEST_RESULTS/phase-3.md`
- `tools/validate-architecture.ps1`

## Out Of Scope

- Real process enumeration through Windows APIs.
- Digital signature or reputation checks.
- Parent process spoofing detection.
- Command-line deobfuscation beyond flag matching.
- Full detection engine scoring.
- Alert generation.
- Remediation or quarantine.
- UI display.
- Persistent storage.
- Real ETW session or callback implementation.

## Approval

The user approved this design direction on 2026-06-27 before implementation planning.
