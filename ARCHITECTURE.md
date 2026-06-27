# SentraEDR Architecture

Date: 2026-06-27
Phase: 8

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
- Phase 2 implements the portable process-event adapter path: synthetic process start/exit records, normalization into `NormalizedTelemetryEvent`, bounded queue delivery, ingestion counters, and component health.
- Real Windows ETW session lifecycle and callbacks remain deferred until the portable ingestion path is stable.
- Does not score threats or remediate.

`engine-process`

- Owns process lineage, executable path analysis, signer metadata, command-line risk signals, and suspicious PowerShell behavior.
- Phase 3 implements observe-only lifecycle state and preliminary process behavior signals from normalized telemetry.
- Does not consume ETW-specific record types; it receives `NormalizedTelemetryEvent` through shared schemas.
- Does not create final alerts or remediation eligibility.
- Does not inspect packets, edit registry, or remediate.

`engine-network`

- Owns outbound connection metadata, DNS metadata, destination rarity, and beacon cadence analysis.
- Phase 5 implements observe-only metadata parsing, small destination history, and preliminary network behavior signals.
- Does not capture packets, resolve DNS, open sockets, or modify firewall state.
- Does not perform remediation or process suspension.

`engine-persistence`

- Owns Run keys, startup folders, scheduled tasks, services, and WMI persistence analysis.
- Phase 4 implements observe-only metadata parsing and preliminary persistence behavior signals from normalized telemetry.
- Does not call Windows Registry, Task Scheduler, Service Control Manager, WMI, or filesystem APIs.
- Does not create final alerts or remediation eligibility.
- Remediation is delegated to `engine-remediation` after policy approval.

`engine-detection`

- Owns correlation, risk scoring, false-positive suppression, alert generation, and ATT&CK mapping.
- Phase 6 implements observe-only heuristic signal correlation, family diversity scoring, MITRE mapping, findings, and observe-only alerts.
- Does not import peer engine crates; it consumes normalized detection input signals and shared schemas.
- Emits verdicts and alerts, not direct destructive actions.

`engine-remediation`

- Owns gated quarantine, process suspension, network isolation hooks, registry backup, persistence rollback, and audit trails.
- Phase 7 implements policy-gated remediation planning, rejection/approval decisions, and audit records.
- Does not execute Windows API calls, filesystem moves, registry writes, firewall changes, process suspension, or deletion.
- Executes only after confidence, policy, and approval checks pass.

`engine-memory`

- Owns memory inspection, suspicious thread starts, and injection indicators.
- Phase 8 implements observe-only memory telemetry metadata analysis for remote thread, executable private memory, unsigned module, section mapping, and memory protection indicators.
- Does not read process memory, dump memory, inject code, call Windows memory APIs, suspend processes, remediate, or create final alerts.

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

## Phase 1 Status

Workspace foundations are implemented. `shared-models`, `shared-ipc`, and `sentra-agent` compile together, and architecture validation enforces the initial dependency direction.

## Phase 2 Status

`engine-etw` now provides observe-only process telemetry ingestion for deterministic tests. The agent runs a synthetic dry run that records two process lifecycle events and logs normalized counts. The implementation intentionally excludes real ETW provider registration, Windows service installation, detection scoring, remediation, and UI streaming.

## Phase 3 Status

`engine-process` now tracks process start and exit telemetry in an in-memory state table and emits preliminary observe-only process signals. Initial signals cover suspicious parent-child process chains, PowerShell encoded command flags, and execution from user-writable paths. Full risk scoring, alert generation, signer reputation, Windows process enumeration, remediation, and UI streaming remain deferred.

## Phase 4 Status

`engine-persistence` now parses persistence metadata from normalized telemetry and emits preliminary observe-only signals for registry Run keys, startup folders, scheduled tasks, services, and WMI subscription indicators. Real registry access, scheduled task APIs, service control APIs, WMI querying, persistence rollback, final detection scoring, alert generation, remediation, and UI streaming remain deferred.

## Phase 5 Status

`engine-network` now parses network metadata from normalized telemetry, tracks small in-memory destination history, and emits preliminary observe-only signals for rare external destinations, suspicious DNS patterns, beacon interval candidates, high-risk ports, and IP-literal outbound connections. Packet capture, WFP, ETW TCP/IP sessions, DNS resolver integration, firewall isolation, final detection scoring, remediation, and UI streaming remain deferred.

## Phase 6 Status

`engine-detection` now correlates preliminary signals into scored findings and observe-only alerts. Initial scoring uses severity hints and signal-family diversity across process, persistence, network, and PowerShell families. Alerts remain remediation-ineligible; policy allowlists, advanced false-positive suppression, production rule loading, remediation, and UI workflows remain deferred.

## Phase 7 Status

`engine-remediation` now evaluates alerts through explicit policy gates and emits auditable remediation decisions. Observe-only alerts, telemetry uncertainty, disabled policy, and below-threshold risk are rejected by policy. Eligible high-risk alerts can produce approval-required remediation plans, but no executor exists yet; quarantine, process suspension, network isolation, registry writes, rollback, and deletion remain unimplemented.

## Phase 8 Status

`engine-memory` now parses normalized memory telemetry metadata and emits preliminary observe-only signals for remote thread creation, executable private memory, unsigned module loads, suspicious section mapping, and executable memory protection changes. This phase is metadata-only; real memory scanning, process handle access, memory reads, dumps, kernel drivers, injection, remediation, final alerting, and UI streaming remain deferred.
