# SentraEDR Architecture

Date: 2026-06-27
Phase: 0

## Purpose

SentraEDR is a Windows-focused, lightweight, behavior-driven EDR platform. The architecture separates telemetry collection, analysis, detection, remediation, and UI concerns so each unit can be built and validated independently.

Phase 0 defines the architecture contract. It does not claim any executable agent exists yet.

## System Shape

```text
Windows telemetry sources
  -> engine-etw
  -> engine-process / engine-network / engine-persistence
  -> engine-detection
  -> engine-remediation
  -> shared-ipc
  -> sentra-ui
```

The flow is intentionally one-directional for security decisions. Detection may emit findings. Remediation may execute only policy-approved, audited actions. The UI may request actions but cannot bypass agent-side validation.

## Planned Crates

`sentra-agent`

- Owns process lifecycle.
- Starts runtimes and engines.
- Loads configuration.
- Hosts local IPC server.
- Does not contain detection rules directly.

`engine-etw`

- Owns ETW provider registration, session lifecycle, raw event parsing, and event normalization.
- Does not score threats or remediate.

`engine-process`

- Owns process lineage, executable path analysis, signer metadata, command-line risk signals, and suspicious PowerShell behavior.
- Does not inspect packets, edit registry, or remediate.

`engine-network`

- Owns outbound connection metadata, DNS metadata, destination rarity, and beacon cadence analysis.
- Does not perform remediation or process suspension.

`engine-persistence`

- Owns Run keys, startup folders, scheduled tasks, services, and WMI persistence analysis.
- Remediation is delegated to `engine-remediation` after policy approval.

`engine-detection`

- Owns correlation, risk scoring, false-positive suppression, alert generation, and ATT&CK mapping.
- Emits verdicts and alerts, not direct destructive actions.

`engine-remediation`

- Owns gated quarantine, process suspension, network isolation hooks, registry backup, persistence rollback, and audit trails.
- Executes only after confidence, policy, and approval checks pass.

`engine-memory`

- Owns memory inspection, suspicious thread starts, and injection indicators.
- Deferred until process and ETW telemetry are stable.

`shared-models`

- Owns schemas for telemetry events, findings, alerts, remediation commands, audit records, and schema versions.
- Must remain dependency-light and deterministic.

`shared-ipc`

- Owns named-pipe framing, transport, serialization, bounded queues, backpressure, and schema negotiation.
- Does not contain detection logic.

`sentra-ui`

- Owns dashboard state, alert review, timeline display, and explicit user action flows.
- Must not import engine internals.

## Dependency Rules

Allowed:

```text
engine-* -> shared-models
engine-* -> shared-ipc
shared-ipc -> shared-models
sentra-agent -> engine-* / shared-*
sentra-ui -> shared-models
```

Forbidden:

```text
shared-models -> engine-*
shared-ipc -> engine-*
engine-process -> engine-network
engine-network -> engine-persistence
engine-detection -> engine-etw
sentra-ui -> engine-*
```

Cross-engine behavior must happen through messages and shared schemas, not direct calls.

## Telemetry Pipeline

Raw telemetry is normalized before it enters analysis engines:

```text
Raw OS event
  -> provider parser
  -> NormalizedTelemetryEvent
  -> priority queue
  -> analyzer
  -> finding
  -> detection correlation
  -> alert
```

Every queue in this pipeline is bounded. Queue pressure is observable and must be logged as a health signal.

## Runtime Boundaries

The runtime model separates:

- ETW ingestion.
- Detection and scoring.
- Network analysis.
- IO-heavy work such as registry, file, SQLite, and quarantine operations.

No runtime may synchronously block another runtime. Blocking OS operations are isolated from hot telemetry ingestion.

## Architecture Invariants

- No unbounded production telemetry channels.
- No direct remediation from telemetry ingestion.
- No UI-owned security verdicts.
- No direct engine-to-engine imports.
- No deletion-first remediation.
- No uncontrolled malware testing.
- No fake detections presented as working protection.

## Phase 0 Status

Architecture is designed but not implemented. Phase 1 must create the workspace and enforce these boundaries through crate dependencies, compile checks, and CI commands.
