# SentraEDR IPC Design

Date: 2026-06-27
Phase: 12

## IPC Choice

SentraEDR uses local Windows named pipes as the first IPC mechanism between the agent and UI. Named pipes fit the local endpoint model, support bidirectional communication, and allow access control through Windows security descriptors.

## IPC Roles

Agent:

- named-pipe server;
- owns policy enforcement;
- validates UI requests;
- sends alerts and state updates.

UI:

- named-pipe client;
- displays telemetry and alerts;
- sends user-approved commands;
- never bypasses agent policy.

## Message Categories

- Health messages.
- Telemetry summaries.
- Alerts.
- User decisions.
- Remediation requests.
- Remediation status.
- Audit records.

## Message Requirements

Each message must include:

- schema version;
- message ID;
- message type;
- timestamp;
- correlation ID where applicable;
- payload;
- validation result after parsing.

Unknown schema versions are rejected or handled through explicit compatibility logic.

## Backpressure

IPC queues are bounded. UI lag must not block detection. Under pressure:

- agent keeps security-critical internal processing first;
- UI receives summaries instead of raw high-volume telemetry;
- low-priority display updates may be coalesced;
- alert and remediation messages stay higher priority.

## Security Controls

- Restrictive pipe ACLs.
- Agent-side authorization for commands.
- Schema validation before dispatch.
- Audit records for every remediation request.
- No direct mapping from UI button to destructive action.

## Serialization

Phase 1 should select a compact Rust-friendly serialization format. JSON may be used only for developer-readable diagnostics outside hot production paths.

## Phase 0 Status

IPC is specified but not implemented. Phase 1 creates the initial crate boundary; functional IPC transport follows after shared schemas exist.

## Phase 12 Status

`shared-ipc` now provides the first functional IPC serialization boundary:

- `IpcEnvelope` carries schema version, message ID, message kind, timestamp, optional correlation ID, and typed payload.
- `IpcPayload` supports health, telemetry summary, alert, user decision, remediation request, remediation status, and audit record categories.
- `encode_frame` and `decode_frame` use a 4-byte big-endian length prefix followed by JSON payload bytes.
- Decode validation rejects unsupported major schema versions, mismatched message kind/payload pairs, incomplete frames, oversized frames, malformed JSON, and trailing bytes.

Named-pipe transport, pipe ACLs, UI streaming, command authorization flow, and live remediation request handling remain deferred.
