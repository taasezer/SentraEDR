# SentraEDR Phase 0 Design

Date: 2026-06-27
Status: Draft for user review
Scope: Research, architecture, threat model, telemetry model, memory model, IPC model, and implementation roadmap.

## Objective

SentraEDR is a lightweight Windows Endpoint Detection and Response platform focused on behavior-based Anti-RAT detection. Phase 0 defines the system contracts before production code is created. The goal is to prevent early architectural drift in the areas that matter most for an EDR: telemetry correctness, safe remediation, bounded resource use, and strict module isolation.

This phase does not create a Rust workspace, service, UI, driver, detector, or remediation implementation. Those are implementation phases. Phase 0 creates the design documents that later phases must follow and validate.

## Current Repository State

The repository currently contains:

- `README.md`: project vision and execution directive.
- `LICENSE`: MIT license.

There is no Rust workspace, no Tauri application, no crate graph, no CI pipeline, and no executable agent. This means Phase 0 is the correct first step because the technical boundaries are still inexpensive to shape.

## Research Baseline

Phase 0 uses the following primary references as design anchors:

- Microsoft ETW documentation: https://learn.microsoft.com/en-us/windows/win32/etw/about-event-tracing
- Microsoft Sysmon documentation: https://learn.microsoft.com/en-us/sysinternals/downloads/sysmon
- MITRE ATT&CK Enterprise Matrix: https://attack.mitre.org/matrices/
- Atomic Red Team: https://github.com/redcanaryco/atomic-red-team
- Microsoft named pipes documentation: https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipes
- Rust for Windows documentation: https://learn.microsoft.com/en-us/windows/dev-environment/rust/rust-for-windows
- YARA-X Rust documentation: https://docs.rs/yara-x/latest/yara_x/
- Tauri 2 documentation: https://v2.tauri.app/

These references support the initial design choices: ETW and Windows Event Log for telemetry, Sysmon as an optional enrichment source, MITRE ATT&CK as the behavioral mapping language, Atomic Red Team as the safe validation catalog, named pipes for local IPC, Rust for Windows API access, YARA-X as a later Rust-native rule engine candidate, and Tauri for a low-footprint desktop UI.

## Phase 0 Design Principles

The system is designed as an event-driven Rust platform. Engines communicate through typed messages, not direct cross-engine calls. The core process must tolerate telemetry bursts without unbounded memory growth. The UI must never own security decisions. Remediation must be gated behind detection confidence, multi-signal validation, audit logging, and user approval in real deployment mode.

The first three system risks are:

- Telemetry overload.
- Wrong remediation.
- Crate coupling breakdown.

Every later phase must explicitly validate these risks before completion.

## Recommended Architecture

The first implementation target is a Rust workspace with clear crate ownership:

- `sentra-agent`: Windows service or agent host that wires engines together.
- `engine-etw`: ETW session lifecycle, provider subscription, event normalization.
- `engine-process`: process creation, parent-child lineage, path and signature metadata analysis.
- `engine-network`: outbound connection, DNS, and beaconing analysis.
- `engine-persistence`: registry, startup folder, scheduled task, service, and WMI persistence analysis.
- `engine-detection`: risk scoring, behavior correlation, alert generation, false-positive suppression.
- `engine-remediation`: gated process isolation, quarantine, rollback, and registry backup.
- `engine-memory`: memory and thread inspection, added after core telemetry is stable.
- `shared-models`: stable event, finding, alert, and command schemas.
- `shared-ipc`: local named-pipe transport, bounded queues, serialization, backpressure, and schema versioning.
- `sentra-ui`: Tauri + React dashboard that receives state and submits explicit user-approved commands.

Dependency direction must remain one way:

```text
engine-* -> shared-ipc -> shared-models
engine-* -> shared-models
sentra-agent -> engine-* / shared-*
sentra-ui -> shared-models / UI IPC client
```

Forbidden dependencies:

- `shared-models` depending on any engine.
- Engine crates directly calling other engine crates.
- UI crates importing engine internals.
- Detection engines invoking remediation actions directly.

## Runtime Model

The agent uses separated runtime responsibilities rather than one unconstrained async runtime:

- ETW ingestion runtime: receives and normalizes telemetry, avoids blocking work.
- Detection runtime: correlates signals and computes risk.
- Network runtime: handles packet or connection analysis when enabled.
- IO/runtime operations: registry reads, file metadata, quarantine storage, SQLite writes.

All hot paths use bounded channels. No production telemetry channel may be unbounded. When pressure rises, low-priority events are dropped or aggregated before high-priority security events.

## Telemetry Model

Initial telemetry sources:

- ETW process creation and exit.
- ETW image load where feasible.
- ETW or Windows Event Log PowerShell activity.
- Registry and persistence-related activity.
- Sysmon event ingestion when Sysmon is installed and configured.
- Network connection metadata from Windows APIs or packet capture where approved.

The canonical normalized event shape is:

```text
NormalizedTelemetryEvent
  schema_version
  event_id
  timestamp_utc
  source
  priority
  process
  subject
  action
  metadata
  confidence_hint
```

The normalized shape is a contract, not an implementation promise for Phase 0. Phase 1 must turn it into Rust types in `shared-models`.

## Detection Model

Detection starts in observe-only mode. The detection engine produces findings and alerts, not destructive actions.

Risk scoring combines multiple signal groups:

- Process lineage: suspicious parent-child chains, LOLBin execution, hidden shell usage.
- PowerShell behavior: encoded commands, download cradle patterns, policy bypass, suspicious child processes.
- Persistence: Run keys, services, scheduled tasks, startup folder, WMI persistence.
- Network: new outbound destinations, beacon-like cadence, suspicious DNS, rare external endpoints.
- Memory: RWX regions, suspicious thread starts, injection indicators; deferred until core telemetry is stable.

Remediation eligibility requires:

- risk score at or above the configured high-confidence threshold;
- at least two independent signal families for high-impact actions;
- rollback material available before registry or file changes;
- explicit user approval for critical actions in real deployment mode.

## Memory And Performance Model

The initial target is less than 150 MB idle memory for the agent and low idle CPU. The design protects this through:

- bounded queues;
- compact shared models;
- binary serialization for hot IPC paths;
- avoidance of JSON in hot telemetry loops;
- aggregation of repetitive low-value events;
- strict logging volume controls;
- deferred loading for heavy analysis modules such as YARA and memory scanning.

Each crate must define a memory budget during implementation. Phase 1 will add build and test scaffolding; later phases add benchmarks.

## IPC Model

Local named pipes are the first IPC mechanism. The design uses message framing and schema versioning so UI and agent can evolve independently.

IPC rules:

- UI may request state, acknowledge alerts, and request remediation.
- Agent validates all UI-originated commands.
- Detection output is advisory until remediation policy accepts it.
- Remediation commands are audited before execution.
- Pipe ACLs must restrict access to the expected local user or service identity.

## Security Model

The platform must avoid becoming a local privilege escalation surface. Security-sensitive boundaries:

- agent service identity;
- named pipe server;
- quarantine directory;
- registry backup store;
- update and rule-loading paths;
- event ingestion parser;
- remediation command handler.

The default development mode is observe-only. Controlled remediation is introduced only after detection confidence, rollback, and audit flows exist.

## Safe Testing Model

SentraEDR must not download or run uncontrolled malware. Validation uses:

- Atomic Red Team tests mapped to ATT&CK;
- EICAR for safe anti-malware pipeline validation where relevant;
- custom benign simulators for process, persistence, and network behaviors;
- isolated Windows virtual machines;
- explicit snapshots before remediation tests.

## Phase Roadmap

Phase 1 initializes the Rust workspace and documentation guardrails. It creates crates, shared models, basic logging, config loading, and CI commands.

Phase 2 implements ETW ingestion for a narrow process-event path, including bounded channels and event normalization.

Phase 3 implements process analysis and initial PowerShell heuristics in observe-only mode.

Phase 4 implements persistence analysis in observe-only mode.

Phase 5 implements network metadata analysis.

Phase 6 implements multi-signal detection scoring and alert generation.

Phase 7 implements gated quarantine and remediation with rollback.

Phase 8 adds memory inspection after earlier telemetry is stable.

Phase 9 adds the Tauri UI.

Phase 10 adds VM test orchestration, Atomic Red Team coverage, regression tests, and performance baselines.

## Phase 0 Completion Criteria

Phase 0 is complete when the repository contains:

- architecture document;
- threat model;
- security model;
- detection model;
- memory model;
- IPC model;
- performance notes;
- task tracker;
- phase report;
- test-results note for Phase 0.

No runtime behavior is claimed in Phase 0.

## Approval Gate

After this spec is committed locally, the user reviews it. Implementation planning begins only after user approval. Push to `main` requires a separate user approval.
