# SentraEDR IPC Design

Date: 2026-06-27
Phase: 17

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

## Phase 13 Status

`shared-ipc` now provides the first in-memory message dispatch boundary:

- `IpcDispatcher` validates envelopes before enqueueing.
- `IpcDispatcherConfig` rejects zero-capacity queues.
- Route-specific queues exist for health, telemetry summary, alert, user decision, remediation request, remediation status, and audit record messages.
- Route statistics track accepted, rejected, and dropped messages.
- Full route queues return the existing `QueueFull` error and preserve bounded backpressure behavior.

Named-pipe transport, pipe ACLs, UI streaming, command authorization, and live remediation handling remain deferred.

## Phase 14 Status

`shared-ipc` now provides a complete-frame intake boundary:

- `IpcFrameIntake` accepts one complete byte frame at a time.
- `accept_frame` uses the existing frame decoder and then dispatches the validated envelope.
- `IpcFrameIntakeStats` tracks accepted frames, decode failures, and dispatch failures.
- Decode failures never enter route queues.
- Dispatch failures are counted after successful decode when validation or queue pressure rejects delivery.

Stream buffering, named-pipe transport, pipe ACLs, UI streaming, command authorization, and live remediation handling remain deferred.

## Phase 15 Status

`shared-ipc` now provides bounded stream assembly before frame intake:

- `IpcStreamAssembler` accepts arbitrary byte chunks.
- Partial frame bytes remain buffered until complete.
- Multiple complete frames in one chunk are emitted in order.
- Oversized length prefixes are rejected before waiting for payload bytes.
- `IpcStreamAssemblerStats` tracks completed frames, buffered bytes, and rejected stream inputs.

Named-pipe transport, pipe ACLs, async read loops, UI streaming, command authorization, and live remediation handling remain deferred.

## Phase 16 Status

`shared-ipc` now provides the IPC processing pipeline composition:

- `IpcPipeline` integrates the stream assembler, frame intake, and dispatcher into a single unit.
- The pipeline accepts raw byte chunks and manages the full flow from assembly to dispatch.
- `IpcPipelineStats` provides an aggregated view of the pipeline's health, tracking chunks received, frames completed, frames accepted, and failures at the stream and intake levels.
- The composition ensures a strictly linear, validated, and bounded data flow from transport-agnostic bytes to typed IPC envelopes.

Named-pipe transport, pipe ACLs, async read loops, UI streaming, command authorization, and live remediation handling remain deferred.

## Phase 17 Status

`sentra-agent` now hosts the first IPC service lifecycle skeleton:

- `IpcConfig` enables or disables IPC processing and bounds dispatcher capacity.
- `IpcService` wraps `IpcPipeline` and accepts raw byte chunks through `process_raw_bytes`.
- The service exposes pipeline stats for chunks, completed frames, accepted frames, stream rejections, decode failures, and dispatch failures.
- A synthetic agent dry run encodes a health envelope, feeds it as fragmented bytes, and verifies delivery into the health route.
- Agent startup logs IPC dry-run counters in observe-only mode.

This phase remains in-memory only. Named-pipe transport, pipe ACLs, async read loops, UI streaming, command authorization, and live remediation handling remain deferred.
