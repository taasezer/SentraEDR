# SentraEDR Architecture Health Report

## 1. Registry Dependency Graph
The `core-registry` now functions as the singular bootstrap authority. 
- Capabilities express dependencies via `CapabilityId`.
- The `BootstrapOrchestrator` verifies these dependencies during startup, preventing the engine from booting in an inconsistent state (e.g. Remediation starting without Storage).

## 2. Infrastructure Coupling
- **Storage:** Completely decoupled via the `EventRepository` and `StorageProvider` traits. Changing underlying SQL flavors or moving to remote gRPC storage requires zero changes to the core engines.
- **Registry:** Fully abstracted. Capabilities merely implement the `Capability` trait to be discovered.

## 3. Storage Throughput & Batching Performance
- Telemetry ingestion writes instantly to the `tokio::sync::mpsc` queue in micro-seconds via `try_send()`.
- The `StoragePipeline` worker thread accumulates events and flushes them in batches, effectively eliminating the disk seek-time bottleneck that plagues synchronous SQLite writes.

## 4. Scalability Assessment
The system is heavily horizontally scalable internally. By isolating ETW parsing onto a dedicated OS thread, Detection onto another, and Storage onto a Tokio async task, the EDR fully saturates multi-core endpoints efficiently without thread contention.

## 5. Readiness for the IPC Layer
Phase 8 has successfully decoupled all the moving parts and wrapped them in an asynchronous event-driven model. The `CapabilityRegistry` allows us to easily drop in an `IpcProvider` in the next phase that communicates over named pipes, feeding commands straight into the Orchestrator or subscribing to Storage events without hacking the core engines.
