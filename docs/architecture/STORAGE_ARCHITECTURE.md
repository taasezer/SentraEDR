# Storage Engine Architecture

The Storage Infrastructure layer records history. It is an append-only event store representing the ground truth of past agent activity. It never acts as the source of truth for *live* system state.

## Schema Versioning & Forward Compatibility
Every persisted domain event encapsulates version metadata:
- `SchemaVersion`: Dictates how the wrapper itself is parsed.
- `EventVersion`: Dictates the structure of the inner domain event payload.
- `ProducerVersion`: The semantic version of the agent/engine that generated the event.

## Pipeline & Batching
The engine enforces asynchronous isolation:
1. `Detection` pushes an event to the `tokio::sync::mpsc` channel. It immediately returns.
2. The `Storage` worker thread loops over the receiver.
3. Once a threshold is reached (e.g. 500 events or 1 second elapsed), the worker batches the events and flushes them to the `StorageProvider`.

## Retention Limits
The Event Store is governed by strict bounded limits. The engine enforces a `RetentionPolicy` that purges (or archives) events exceeding maximum age or maximum disk footprint.

## Storage Provider Decoupling
The core system communicates through the `EventRepository` trait. Replacing a local SQLite implementation with a remote gRPC logger requires writing a new struct that implements the trait, and registering it with the `CapabilityRegistry`. Zero business logic changes are required.
