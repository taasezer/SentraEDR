# Phase 13 IPC Dispatcher Design

Date: 2026-06-27
Status: Approved by continuation request

## Goal

Add an in-memory IPC dispatcher to `shared-ipc` that routes validated `IpcEnvelope` messages into bounded per-category queues. This creates the dispatch boundary needed before any Windows named-pipe transport is added.

## Scope

Phase 13 implements:

- `IpcDispatcher` as an in-memory router;
- per-route bounded queues for health, telemetry summary, alerts, user decisions, remediation requests, remediation status updates, and audit records;
- non-blocking dispatch through existing `BoundedSender::try_send`;
- route statistics for accepted, rejected, and dropped messages;
- validation before enqueueing.

Phase 13 does not implement:

- named-pipe server/client transport;
- Windows pipe ACLs;
- UI streaming;
- command authorization workflow;
- remediation execution;
- malware or Atomic Red Team execution;
- VM orchestration;
- deployment or release signing.

## Architecture

`shared-ipc` gains a focused `dispatcher` module. The module owns only routing decisions and queue state. It does not parse bytes; frame decoding remains in `frame`. It does not execute message payloads; consumers receive typed envelopes from route-specific receivers.

```text
IpcEnvelope
  -> validate()
  -> IpcDispatcher::dispatch()
  -> bounded route queue
  -> consumer try_recv()/recv()
```

The dispatcher is created with `IpcDispatcher::new(IpcDispatcherConfig)`. The config defines per-route capacity and rejects zero capacity. `dispatch` validates the envelope and routes it by `IpcMessageKind`. Queue pressure is reported through existing `IpcError::QueueFull`, and the dispatcher records drops through queue metrics plus route statistics.

## Safety

This phase is passive and deterministic. It does not connect to the OS, open pipe handles, mutate host state, run tools, or execute remediation. UI-originated decisions and remediation requests are only queued as data.

## Testing

Tests cover:

- alerts route to the alert queue;
- remediation requests route to the remediation request queue;
- kind/payload mismatches are rejected before enqueueing;
- route queue pressure returns `QueueFull` and increments stats;
- zero dispatcher capacity is rejected.

Final verification must include the full workspace quality gate runner.
