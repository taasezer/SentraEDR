# Phase 4 Persistence Monitoring Design

Date: 2026-06-27
Status: Approved for implementation planning
Branch target: `Omer`

## Goal

Phase 4 adds a lightweight `engine-persistence` crate that consumes normalized telemetry metadata and emits observe-only persistence behavior signals.

This phase does not implement real registry reads or writes, scheduled task APIs, service control APIs, WMI queries, filesystem scanning, persistence rollback, final detection scoring, alerting, remediation, named-pipe IPC, UI streaming, or real ETW callbacks.

## Context

Phase 2 added process telemetry ingestion. Phase 3 added process lifecycle state and preliminary process behavior signals. Phase 4 adds a persistence-focused analyzer that stays independent from process and ETW engine internals.

```text
NormalizedTelemetryEvent
  -> engine-persistence
  -> PersistenceAnalysisReport
  -> future engine-detection
```

`engine-persistence` consumes shared schemas only. It reads structured metadata from telemetry events and converts suspicious persistence-related changes into preliminary signals for a future detection phase.

## Crate Boundary

Create:

- `crates/engine-persistence`

Allowed dependencies:

- `shared-models`
- `thiserror` only if a specific error type is required

Forbidden dependencies:

- `sentra-agent`
- `sentra-ui`
- peer engines such as `engine-etw`, `engine-process`, `engine-network`, `engine-detection`, and `engine-remediation`

The engine must not import Windows API bindings in Phase 4. It analyzes already-normalized telemetry.

## Inputs

Input type:

- `NormalizedTelemetryEvent`

Primary action:

- `TelemetryAction::RegistryChanged`

Phase 4 may also accept other actions if they carry persistence metadata, but the initial implementation should keep tests centered on metadata rather than adding new OS collection behavior.

Expected metadata keys:

- `persistence.kind`
- `persistence.path`
- `persistence.value`
- `persistence.operation`

Optional metadata keys:

- `persistence.image_path`
- `persistence.command`
- `persistence.user`
- `persistence.source`

Events without persistence metadata are ignored and counted as ignored telemetry.

## Outputs

Primary output:

- `PersistenceAnalysisReport`

The report contains:

- number of events observed;
- number of persistence events handled;
- number of ignored events;
- emitted persistence signals;
- optional component health.

Signal output:

- `PersistenceSignal`

Signals are preliminary observations, not findings or alerts. Future detection correlation can combine them with process, network, PowerShell, and reputation evidence.

## Persistence Event Model

`PersistenceEvent` represents normalized persistence metadata:

- kind;
- path;
- value;
- operation;
- optional image path;
- optional command;
- optional user;
- supporting telemetry event ID;
- observed timestamp.

Persistence kind values:

- `RegistryRunKey`
- `StartupFolder`
- `ScheduledTask`
- `Service`
- `WmiSubscription`
- `Unknown`

The parser should use conservative string matching from metadata. It must not claim that it verified registry ACLs, task XML, service state, or WMI repository contents.

## Initial Signals

Phase 4 implements five deterministic observe-only signals.

### Registry Run Key Persistence

Emit `registry_run_key_persistence` when metadata indicates a Run or RunOnce key modification.

Initial case-insensitive path fragments:

- `\software\microsoft\windows\currentversion\run`
- `\software\microsoft\windows\currentversion\runonce`

Severity hint: `High`

### Startup Folder Persistence

Emit `startup_folder_persistence` when metadata indicates startup folder persistence.

Initial case-insensitive path fragments:

- `\start menu\programs\startup\`
- `\startup\`

Severity hint: `Medium`

### Scheduled Task Persistence

Emit `scheduled_task_persistence` when metadata kind is scheduled task or path contains scheduled task context.

Initial case-insensitive markers:

- `scheduled_task`
- `\microsoft\windows\task scheduler\`
- `\system32\tasks\`

Severity hint: `Medium`

### Service Persistence

Emit `service_persistence` when metadata kind is service or path indicates service creation/path modification.

Initial case-insensitive markers:

- `service`
- `\system\currentcontrolset\services\`

Severity hint: `High`

### WMI Persistence

Emit `wmi_persistence` when metadata kind or path indicates WMI permanent event subscription persistence.

Initial case-insensitive markers:

- `wmi`
- `__eventfilter`
- `commandlineeventconsumer`
- `__filtertoconsumerbinding`

Severity hint: `High`

## Scoring

Phase 4 does not perform final risk scoring.

Each `PersistenceSignal` can include a local severity hint:

- `Low`
- `Medium`
- `High`

These hints help future detection work but do not create alerts, findings, or remediation eligibility.

## Error Handling

Malformed or irrelevant telemetry is not fatal.

The engine should:

- increment ignored counts for events without persistence metadata;
- tolerate missing optional fields;
- avoid panics in normal analysis paths;
- return a report even when no signals are emitted.

## Security Boundaries

`engine-persistence` must not:

- edit registry keys;
- create, modify, or delete scheduled tasks;
- create, modify, or delete services;
- query or mutate WMI subscriptions;
- create startup folder files;
- quarantine files;
- create alerts directly;
- import ETW-specific or process-engine-specific types;
- import detection, remediation, network, memory, agent, or UI crates.

Signals are evidence for future correlation, not final verdicts.

## Performance Constraints

The implementation should stay allocation-light:

- metadata extraction uses existing `TelemetryMetadata::get`;
- matching uses lowercase strings only when needed;
- no regex dependency is required for Phase 4;
- no unbounded channel is introduced;
- no persistent store is introduced.

Synthetic test data is small and deterministic. High-volume registry or Windows Event Log ingestion benchmarks are future work.

## Testing Strategy

Tests must be TDD-first.

Required tests:

- Run key metadata emits `registry_run_key_persistence`;
- startup folder metadata emits `startup_folder_persistence`;
- scheduled task metadata emits `scheduled_task_persistence`;
- service metadata emits `service_persistence`;
- WMI metadata emits `wmi_persistence`;
- irrelevant telemetry is ignored without panic;
- architecture validation rejects forbidden `engine-persistence` dependencies.

Agent integration should have a synthetic dry-run test after the core engine is implemented.

## Documentation Updates

Phase 4 implementation must update:

- `ARCHITECTURE.md`
- `PERFORMANCE_NOTES.md`
- `TASKS.md`
- `PHASE_REPORTS/phase-4.md`
- `TEST_RESULTS/phase-4.md`
- `tools/validate-architecture.ps1`

## Out Of Scope

- Windows Registry API access.
- Scheduled task enumeration or task XML parsing.
- Service Control Manager API access.
- WMI repository querying.
- Startup folder filesystem scanning.
- Persistence rollback.
- Full detection scoring.
- Alert generation.
- Remediation or quarantine.
- UI display.
- Persistent storage.
- Real ETW session or callback implementation.

## Approval

The user approved this design direction on 2026-06-27 before implementation planning.
