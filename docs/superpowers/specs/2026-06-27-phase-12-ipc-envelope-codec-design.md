# Phase 12 IPC Envelope Codec Design

Date: 2026-06-27
Status: Approved by continuation request

## Goal

Add a deterministic IPC message envelope and frame codec to `shared-ipc` so the agent and UI can later exchange typed messages over Windows named pipes without coupling either side to transport internals.

## Scope

Phase 12 implements serialization boundaries only:

- message envelope metadata;
- message kind classification;
- payload variants for health, telemetry summary, alerts, user decisions, remediation requests, remediation status, and audit records;
- length-prefixed JSON frame encoding and decoding;
- schema version and frame-size validation.

Phase 12 does not implement a named-pipe server, named-pipe client, Windows ACLs, UI streaming, remediation execution, malware testing, VM orchestration, deployment, or release signing.

## Architecture

`shared-ipc` gains two focused modules:

- `message`: typed IPC envelope, message identifiers, message kinds, payload variants, and validation.
- `frame`: length-prefixed JSON encoding and decoding for one complete message frame.

The envelope carries `SchemaVersion`, `MessageId`, `MessageKind`, `Timestamp`, optional correlation ID, and `IpcPayload`. The payload uses existing `shared-models` types where available and small IPC-specific structs where a full engine model does not exist yet.

The frame format is:

```text
4-byte big-endian payload length
UTF-8 JSON payload bytes
```

The decoder rejects incomplete frames, unknown major schema versions, mismatched message kind/payload combinations, and frames larger than a fixed maximum. The codec is deterministic and in-memory only.

## Safety

This phase is passive infrastructure. It never opens sockets or named pipes, starts services, modifies the host, executes remediation, or runs external security tooling. It only validates and serializes Rust data structures.

## Testing

Tests cover:

- alert message round-trip through the frame codec;
- correlation IDs surviving encode/decode;
- oversized frame rejection;
- incomplete frame rejection;
- message kind and payload mismatch rejection;
- unsupported major schema version rejection.

Final validation must include the full workspace quality gate runner.
