# ADR 0024: Cancellation Strategy

## Status
Accepted

## Decision
The platform utilizes `tokio_util::sync::CancellationToken` hierarchically. A root platform token spawns children (`Infrastructure`, `Detection`, `Storage`). Cancelling a parent automatically cascades the signal to all children. This guarantees deterministic teardown of deep asynchronous call stacks without relying on manual Drop implementations.
