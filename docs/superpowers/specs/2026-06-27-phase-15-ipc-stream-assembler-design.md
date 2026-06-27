# Phase 15 IPC Stream Assembler Design

Date: 2026-06-27
Status: Approved by continuation request

## Goal

Add a bounded in-memory IPC stream assembler that accepts arbitrary byte chunks and emits complete length-prefixed frames for the existing frame intake layer.

## Scope

Phase 15 implements:

- `IpcStreamAssembler` as a deterministic in-memory chunk buffer;
- partial frame buffering across multiple chunks;
- extraction of multiple complete frames from one chunk;
- oversized frame rejection based on the existing maximum frame payload size;
- bounded buffered byte accounting and simple assembler statistics.

Phase 15 does not implement:

- Windows named-pipe server/client transport;
- pipe ACLs;
- async read loops;
- UI streaming;
- command authorization workflow;
- remediation execution;
- malware or Atomic Red Team execution;
- VM orchestration;
- deployment or release signing.

## Architecture

`shared-ipc` gains a focused `stream` module:

```text
byte chunks
  -> IpcStreamAssembler::push_bytes()
  -> complete frame bytes
  -> IpcFrameIntake::accept_frame()
  -> IpcDispatcher
```

The assembler understands only the 4-byte big-endian frame prefix already used by `encode_frame`. It does not deserialize payloads and does not dispatch messages. It keeps incomplete bytes in a bounded buffer and emits complete frame byte vectors when enough data arrives.

The maximum buffered incomplete frame is `4 + MAX_FRAME_PAYLOAD_BYTES`. Prefixes that claim a larger payload are rejected before waiting for more data.

## Safety

This phase is passive infrastructure. It does not open OS handles, run background tasks, execute payloads, authorize commands, or mutate host state. Remediation requests remain opaque bytes until later decode and dispatch layers handle them as data.

## Testing

Tests cover:

- a frame split across chunks is emitted only after complete;
- two frames in one chunk are both emitted;
- an oversized length prefix is rejected before payload buffering;
- partial bytes remain buffered and are reported in stats.

Final verification must include the full workspace quality gate runner.
