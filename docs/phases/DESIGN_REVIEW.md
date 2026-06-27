# Design Review: Phase 8 (Storage Infrastructure & Registry)

## Architectural Boundaries
- **Infrastructure Isolation:** Both `core-registry` and `infrastructure-storage` sit exactly where they belong: at the base layer. They do not depend on `engine-detection`, `engine-remediation`, or telemetry modules.
- **Dependency Resolution:** The `CapabilityRegistry` exposes explicit dependency vectors, preventing race conditions or unresolved dependencies during agent startup.

## ADR Alignment
- ADR-0008 (Immutable Event Store) is fulfilled via `tokio::sync::mpsc` channels and a strict `StorageProvider` interface that only exposes `write_batch` and `query_range`, lacking any update or delete mutations.
- Schema versioning metadata is firmly embedded in the `PersistedEvent` struct.

## Failure Constraints
- The `StoragePipeline::enqueue()` utilizes `try_send()`. This explicitly guarantees that if the storage worker thread crashes or stalls (due to disk I/O blocks), the telemetry parser threads will simply drop events (or buffer them depending on future queue implementation) rather than deadlocking the entire agent.

**Decision: PASS**
