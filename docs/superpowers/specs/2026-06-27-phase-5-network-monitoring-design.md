# Phase 5 Network Monitoring Design

Date: 2026-06-27
Status: Approved for implementation planning
Branch target: `Omer`

## Goal

Phase 5 adds a lightweight `engine-network` crate that consumes normalized network telemetry metadata, keeps small destination history, and emits observe-only network behavior signals.

This phase does not implement packet capture, Windows Filtering Platform hooks, ETW TCP/IP sessions, DNS resolver integration, firewall isolation, socket inspection, final detection scoring, alerting, remediation, named-pipe IPC, UI streaming, or real network collection.

## Context

Previous phases added process, persistence, and telemetry foundations. Phase 5 adds network-focused analysis while staying independent from ETW, process, and persistence engine internals.

```text
NormalizedTelemetryEvent
  -> engine-network
  -> NetworkAnalysisReport
  -> future engine-detection
```

`engine-network` consumes shared schemas only. It reads structured metadata from telemetry events and turns suspicious network observations into preliminary signals for a future detection phase.

## Crate Boundary

Create:

- `crates/engine-network`

Allowed dependencies:

- `shared-models`
- `thiserror` only if a specific error type is required

Forbidden dependencies:

- `sentra-agent`
- `sentra-ui`
- peer engines such as `engine-etw`, `engine-process`, `engine-persistence`, `engine-detection`, and `engine-remediation`

The engine must not import Windows API bindings or networking libraries in Phase 5. It analyzes already-normalized telemetry metadata.

## Inputs

Input type:

- `NormalizedTelemetryEvent`

Primary action:

- `TelemetryAction::NetworkConnectionObserved`

Expected metadata keys:

- `network.remote_ip`
- `network.remote_port`
- `network.protocol`
- `network.direction`

Optional metadata keys:

- `network.domain`
- `network.process_id`
- `network.local_ip`
- `network.local_port`
- `network.bytes_out`
- `network.bytes_in`

Events without network metadata are ignored and counted as ignored telemetry.

## Outputs

Primary output:

- `NetworkAnalysisReport`

The report contains:

- number of events observed;
- number of network events handled;
- number of ignored events;
- current tracked destination count;
- emitted network signals;
- optional component health.

Signal output:

- `NetworkSignal`

Signals are preliminary observations, not findings or alerts. Future detection correlation can combine them with process, persistence, PowerShell, and reputation evidence.

## Network Event Model

`NetworkEvent` represents normalized network metadata:

- remote IP;
- remote port;
- protocol;
- direction;
- optional domain;
- optional process ID;
- optional local IP;
- optional local port;
- supporting telemetry event ID;
- observed timestamp.

The parser should use conservative string and integer parsing from metadata. It must not claim that it verified packet payloads, DNS resolution, connection ownership, or firewall state.

## Destination History

`NetworkDestinationHistory` stores small in-memory observations by destination key.

Destination key:

- domain when present;
- otherwise remote IP.

Tracked values:

- destination key;
- remote IP;
- remote port;
- first observed timestamp;
- last observed timestamp;
- observation count;
- recent interval seconds between observations.

The initial implementation can keep all observed synthetic state in memory. Retention limits and memory pressure policies are future work before high-volume network telemetry is claimed.

## Initial Signals

Phase 5 implements five deterministic observe-only signals.

### Rare External Destination

Emit `rare_external_destination` when a connection is outbound and the destination is not private/local or on the small built-in benign list.

Initial benign domains:

- `localhost`
- `microsoft.com`
- `windowsupdate.com`

Initial benign IP ranges:

- `127.0.0.0/8`
- `10.0.0.0/8`
- `172.16.0.0/12`
- `192.168.0.0/16`
- `::1`

Severity hint: `Medium`

### Suspicious DNS Pattern

Emit `suspicious_dns_pattern` when the domain has suspicious structure.

Initial case-insensitive checks:

- domain length greater than 80 characters;
- at least 5 labels;
- contains `duckdns`;
- contains `no-ip`;
- ends with `.tk`;
- ends with `.top`.

Severity hint: `Medium`

### Beacon Interval Candidate

Emit `beacon_interval_candidate` when the same destination is observed at least 3 times and the last two intervals are equal.

The first implementation uses exact second equality from deterministic synthetic timestamps. It must not claim statistical beacon detection.

Severity hint: `High`

### High-Risk Port

Emit `high_risk_port` when the remote port is in a small high-risk list.

Initial ports:

- `4444`
- `1337`
- `6667`
- `31337`

Severity hint: `Medium`

### IP Literal Connection

Emit `ip_literal_connection` when a connection is outbound to a public IP and no domain metadata is present.

Severity hint: `Low`

## Scoring

Phase 5 does not perform final risk scoring.

Each `NetworkSignal` can include a local severity hint:

- `Low`
- `Medium`
- `High`

These hints help future detection work but do not create alerts, findings, or remediation eligibility.

## Error Handling

Malformed or irrelevant telemetry is not fatal.

The engine should:

- increment ignored counts for events without required network metadata;
- tolerate missing optional fields;
- treat invalid ports as ignored events;
- avoid panics in normal analysis paths;
- return a report even when no signals are emitted.

## Security Boundaries

`engine-network` must not:

- capture packets;
- inspect packet payloads;
- open sockets;
- modify firewall rules;
- isolate network access;
- resolve DNS;
- create alerts directly;
- import ETW-specific, process-engine-specific, or persistence-engine-specific types;
- import detection, remediation, memory, agent, or UI crates.

Signals are evidence for future correlation, not final verdicts.

## Performance Constraints

The implementation should stay allocation-light:

- metadata extraction uses existing `TelemetryMetadata::get`;
- matching uses lowercase strings only when needed;
- no regex dependency is required for Phase 5;
- no unbounded channel is introduced;
- no persistent store is introduced.

Synthetic test data is small and deterministic. High-volume network telemetry, DNS, packet, or connection benchmarks are future work.

## Testing Strategy

Tests must be TDD-first.

Required tests:

- public outbound destination emits `rare_external_destination`;
- suspicious domain emits `suspicious_dns_pattern`;
- repeated equal intervals emit `beacon_interval_candidate`;
- high-risk port emits `high_risk_port`;
- public IP without domain emits `ip_literal_connection`;
- private/local telemetry is not flagged as rare external;
- irrelevant telemetry is ignored without panic;
- architecture validation rejects forbidden `engine-network` dependencies.

Agent integration should have a synthetic dry-run test after the core engine is implemented.

## Documentation Updates

Phase 5 implementation must update:

- `ARCHITECTURE.md`
- `PERFORMANCE_NOTES.md`
- `TASKS.md`
- `PHASE_REPORTS/phase-5.md`
- `TEST_RESULTS/phase-5.md`
- `tools/validate-architecture.ps1`

## Out Of Scope

- Packet capture.
- Windows Filtering Platform.
- ETW TCP/IP provider sessions.
- DNS resolver integration.
- Firewall rule changes.
- Network isolation.
- Connection ownership verification through OS APIs.
- Payload inspection.
- Full detection scoring.
- Alert generation.
- Remediation or quarantine.
- UI display.
- Persistent storage.
- Real ETW session or callback implementation.

## Approval

The user approved this design direction on 2026-06-27 before implementation planning.
