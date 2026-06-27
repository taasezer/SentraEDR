# Phase 14 IPC Frame Intake Design

Date: 2026-06-27
Status: Approved by continuation request

## Goal

Add an in-memory IPC frame intake layer that accepts complete byte frames, decodes them with the existing frame codec, validates the resulting envelope, and dispatches it through `IpcDispatcher`.

## Scope

Phase 14 implements:

- `IpcFrameIntake` as a deterministic in-memory adapter;
- `IpcFrameIntakeStats` for accepted, decode-failed, and dispatch-failed frame counts;
- frame decode failure accounting;
- dispatch failure accounting for full route queues and validation errors;
- access to the underlying dispatcher so tests and future consumers can read routed messages.

Phase 14 does not implement:

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

`shared-ipc` gains a focused `intake` module:

```text
complete frame bytes
  -> IpcFrameIntake::accept_frame()
  -> decode_frame()
  -> IpcDispatcher::dispatch()
  -> bounded route queue
```

`IpcFrameIntake` owns an `IpcDispatcher`. Decode failures are counted before dispatch. Dispatch failures are counted after successful decode when route validation or queue pressure rejects the envelope.

The intake accepts only complete frames. Buffer assembly from streams remains a future transport concern.

## Safety

This phase is passive infrastructure. It does not open named pipes, create services, read sockets, mutate host state, execute payloads, or approve remediation. Remediation request frames are decoded and queued as data only.

## Testing

Tests cover:

- encoded alert frames route to the alert queue;
- malformed frames increment decode failure count;
- full dispatcher queues increment dispatch failure count;
- remediation request frames are queued but not executed.

Final verification must include the full workspace quality gate runner.
