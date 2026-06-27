# SentraEDR IPC Design

## Communication Architecture
SentraEDR utilizes Named Pipes for inter-process communication (IPC). The architecture mandates that engines communicate via the `shared-ipc` layer, ensuring strict boundaries.

## Rules of Engagement
- **One-Way Data Flow:** Telemetry flows from `engine-etw` -> Analysis Engines -> `engine-detection`.
- **No Direct Crate Coupling:** Engines never call each other directly. All cross-boundary communication uses strictly typed, versioned message schemas from `shared-models`.
- **Backpressure Handling:** The IPC layer implements backpressure. If a receiver is overloaded, the sender must adapt (e.g., shed load) without blocking the entire telemetry pipeline.

## IPC Schemas
All messages transmitted over IPC must be serialized using a lightweight binary protocol and validated against structures defined in `shared-models`. This forms an immutable contract between components.

## UI Communication
The `dashboard-ui` (Tauri) communicates with the backend engine via a designated IPC pipe or secure local socket, sending read-only queries and receiving alert streams. Remediation commands from the UI require explicit confirmation mechanisms and schema validation.
