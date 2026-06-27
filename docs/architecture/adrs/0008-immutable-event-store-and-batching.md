# ADR 0008: Immutable Event Store & Batching

## Status
Accepted

## Context
EDR telemetry generates immense volumes of data (thousands of events per second). Writing each event to disk synchronously will block the detection thread and cause the ETW ring buffers to overflow. Furthermore, past events must never be altered to maintain forensic integrity.

## Decision
The Storage Engine functions as an Append-Only Event Store. It does not update rows; it only inserts them. To solve the I/O bottleneck, the pipeline utilizes asynchronous batching (via Tokio `mpsc` channels), grouping events by size or time before flushing to the `StorageProvider` in a single transaction.

## Alternatives Considered
- **Synchronous Writes:** Immediately bottlenecks the ETW parser.
- **Mutable State DB:** Updating rows (e.g. `UPDATE Process SET exit_time = ...`) requires complex locks and query logic that breaks the Event Sourcing paradigm.

## Trade-offs
- *Pros:* Maximum throughput. Guarantees telemetry ingestion is never blocked by disk I/O.
- *Cons:* If the agent crashes, events trapped in the asynchronous buffer memory are lost.

## Consequences
The engine explicitly prioritizes ingestion performance and failure isolation over guaranteed durability of the absolute latest millisecond of data.
