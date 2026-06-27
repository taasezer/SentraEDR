# SentraEDR Phase 2 ETW Process Ingestion Design

Date: 2026-06-27
Status: Approved by user on 2026-06-27
Scope: Narrow ETW process telemetry ingestion design for process start and process exit events.

## Objective

Phase 2 introduces the first telemetry engine boundary for SentraEDR. The goal is to add an `engine-etw` crate that can normalize process start and process exit telemetry into `shared-models::NormalizedTelemetryEvent` and deliver those events through bounded queues.

Phase 2 does not implement detection scoring, remediation, named-pipe IPC, UI streaming, registry monitoring, PowerShell parsing, image-load parsing, network monitoring, malware simulation, or Windows service installation.

## Phase 1 Baseline

Phase 1 created:

- Rust workspace root.
- `shared-models` for telemetry, findings, health, and remediation schemas.
- `shared-ipc` for bounded queue primitives.
- `sentra-agent` with observe-only config and logging.
- Architecture boundary validation.

Phase 2 builds on those crates and must preserve the existing dependency rules:

```text
engine-etw -> shared-models
engine-etw -> shared-ipc
sentra-agent -> engine-etw
shared-models -> no Sentra crate dependencies
shared-ipc -> shared-models only
```

## Technical References

Phase 2 uses the following official Microsoft ETW references:

- ETW overview: https://learn.microsoft.com/en-us/windows/win32/etw/about-event-tracing
- Consuming ETW events: https://learn.microsoft.com/en-us/windows/win32/etw/consuming-events
- `StartTraceW`: https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-starttracew
- `OpenTraceW`: https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-opentracew
- `ProcessTrace`: https://learn.microsoft.com/en-us/windows/win32/api/evntrace/nf-evntrace-processtrace
- `EVENT_RECORD`: https://learn.microsoft.com/en-us/windows/win32/api/evntcons/ns-evntcons-event_record

The design follows the ETW producer, session, and consumer model. Real-time callback work must remain minimal so ETW ingestion does not become a telemetry overload source.

## Design Approach

The selected approach is adapter-first ETW ingestion:

- `engine-etw` owns ETW event-source abstractions.
- A synthetic event source is used for deterministic tests.
- A Windows ETW event source is isolated behind Windows-specific modules and can be feature-gated if needed.
- The normalizer converts process lifecycle records into shared telemetry events.
- The ingestion pipeline publishes normalized events into `shared-ipc` bounded queues.
- Queue pressure is surfaced through health metrics.

This approach keeps Phase 2 testable on the current workstation while preserving the architecture needed for real ETW integration.

## Planned Crate

`engine-etw`

Responsibilities:

- Define ETW process lifecycle input records.
- Define an `EtwEventSource` trait.
- Provide a synthetic source for tests and replay-like validation.
- Normalize process start and process exit records into `NormalizedTelemetryEvent`.
- Send normalized events through `shared-ipc` bounded queues.
- Track ingestion counters: received, normalized, dropped, and failed.
- Provide component health snapshots.

Non-responsibilities:

- Threat scoring.
- Process reputation.
- Remediation.
- Registry monitoring.
- PowerShell-specific parsing.
- UI delivery.
- Named-pipe transport.
- Windows service hosting.

## Event Model

Phase 2 process input records:

```text
EtwProcessRecord
  timestamp
  process_id
  parent_process_id
  image_path
  command_line
  event_kind
```

Supported event kinds:

- process start;
- process exit.

Normalized output:

```text
NormalizedTelemetryEvent
  source = Etw
  priority = Medium for process start
  priority = Low for process exit
  action = ProcessStarted or ProcessExited
  process = ProcessIdentity
  confidence_hint = 100 for synthetic test records and parsed ETW records with required fields
```

If a required field is unavailable, Phase 2 must still emit a structurally valid event when the process ID and timestamp are known, but confidence should be lower and missing values should remain `None`.

## Runtime And Queue Model

Phase 2 adds a narrow ingestion runner:

```text
EtwEventSource
  -> EtwProcessNormalizer
  -> shared-ipc bounded queue
  -> sentra-agent observe-only drain in tests or dry run
```

The queue remains bounded. A full queue must not allocate unbounded memory. Drop counts must be visible through queue health and ingestion counters.

The ETW callback path must not perform detection, database writes, UI updates, blocking file IO, or remediation checks.

## Windows Compatibility

The current workstation validates Rust with `stable-x86_64-pc-windows-gnu` because MSVC `link.exe` was unavailable during Phase 1. Real Windows ETW API binding may require revisiting MSVC Build Tools or carefully selecting Windows API bindings that work with the active toolchain.

Phase 2 implementation should therefore split the work:

- portable core: event records, normalizer, synthetic source, ingestion runner, metrics, tests;
- Windows ETW source: isolated module compiled only when the selected Windows API dependency and linker environment are available.

The portable core must pass all workspace tests even if the real Windows ETW source is not active.

## Error Handling

Phase 2 errors must distinguish:

- source start failure;
- source read failure;
- event normalization failure;
- bounded queue full;
- queue receiver closed;
- unsupported provider or event kind.

Errors must not panic the agent during observe-only ingestion. They should update health state and counters.

## Security Constraints

Phase 2 is observe-only.

The ETW engine must not:

- suspend or kill processes;
- modify registry values;
- quarantine files;
- open named-pipe command surfaces;
- execute test payloads;
- claim malware detection.

Telemetry payloads are untrusted. Paths and command lines must be stored as data, not executed or interpreted as commands.

## Performance Constraints

Phase 2 must keep hot-path work small:

- no unbounded channels;
- no detection scoring in ingestion;
- no heavy string processing in callback-facing code;
- no raw high-volume event logging by default;
- queue full behavior must return an explicit error and increment drop metrics.

Performance validation in Phase 2 is limited to bounded queue and synthetic burst tests. Real ETW throughput benchmarking is deferred until the Windows ETW source is active.

## Testing Strategy

Tests must use synthetic process events. They should prove:

- process start records normalize to `TelemetryAction::ProcessStarted`;
- process exit records normalize to `TelemetryAction::ProcessExited`;
- process identity fields are preserved;
- confidence is clamped to the `0..=100` range where configurable;
- bounded queue overflow increments drop counters;
- ingestion runner drains a finite synthetic source without blocking;
- component health reports degraded state after queue pressure.

No uncontrolled malware, Atomic Red Team, or live ETW provider is required for Phase 2 unit tests.

## Documentation Updates

Phase 2 must update:

- `TASKS.md`;
- `ARCHITECTURE.md`;
- `PERFORMANCE_NOTES.md`;
- `TEST_RESULTS/phase-2.md`;
- `PHASE_REPORTS/phase-2.md`.

`DETECTION_ENGINE.md` should not claim new detection capability in Phase 2.

## Completion Criteria

Phase 2 is complete when:

- `engine-etw` exists in the workspace;
- synthetic process lifecycle events normalize into shared telemetry events;
- bounded queue delivery is tested;
- queue overflow is tested;
- agent can run an observe-only synthetic ingestion dry run;
- architecture validation passes;
- `cargo fmt --all -- --check` passes;
- `cargo clippy --workspace --all-targets -- -D warnings` passes;
- `cargo test --workspace` passes;
- Phase 2 docs and test results are updated.

## Deferred Work

Deferred to later phases:

- real kernel logger session lifecycle hardening;
- image-load ETW parsing;
- registry ETW parsing;
- PowerShell ETW parsing;
- network ETW parsing;
- Sysmon ingestion;
- detection correlation;
- remediation actions;
- UI streaming.

These are deferred to keep Phase 2 narrow, testable, and consistent with the Phase 0 roadmap.

## Approval Gate

After this spec is committed locally, the user reviews it. Implementation planning begins only after user approval. GitHub pushes target the `Omer` branch unless the user changes the target.
