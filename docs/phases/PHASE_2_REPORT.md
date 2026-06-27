# Phase 2: ETW Telemetry Engine Report

## Completed Work
- **Documentation Updates:** Created `ETW_NOTES.md` outlining provider targets and boundaries. Updated `MEMORY_MODEL.md` to define strict zero-heap-allocation rules for the ingestion hot path. Created `PERFORMANCE_NOTES.md` documenting our atomic metrics framework.
- **Shared Models (`shared-models`):** Defined the platform-agnostic `NormalizedTelemetryEvent`. This model contains zero ETW-specific fields, allowing future Sysmon/Linux agents to utilize the same pipeline.
- **Dependency Minimization:** Restricted `tokio` to `rt`, `sync`, and `time` features. Restricted the `windows` crate strictly to ETW and foundational elements (`Win32_System_Diagnostics_Etw`, `Win32_System_Threading`).
- **ETW Engine Modules (`engine-etw`):**
  - `metrics.rs`: Thread-safe, lock-free atomic counters for ingestion tracking (received, parsed, normalized, dropped, latencies).
  - `provider.rs`: GUID constants and `ProviderConfig` for process and PowerShell telemetry.
  - `session.rs`: The lifecycle abstraction for ETW tracking sessions.
  - `parser.rs`: Defines `RawEtwEvent`. Isolates the C-struct pointers (`EVENT_RECORD`). Parsing operates with zero unneeded cloning.
  - `normalizer.rs`: Independently translates `RawEtwEvent` to `NormalizedTelemetryEvent`.
- **Unit Testing:** Implemented synthetic unit tests in `normalizer.rs` to prove the logic converts events correctly without relying on a live OS session.

## Failure Boundaries Tested
- Parser logic explicitly returns `Option<RawEtwEvent>`, isolating failures without crashing the dedicated ETW OS thread.
- Normalizer gracefully handles unknown event schemas without halting the Tokio task processing the channels.

## Next Phase (Phase 3: Process Monitoring Engine)
- We will wire the output of `engine-etw` into `engine-process`.
- We will build the token inspection and process enrichment logic.
- We will detect unsigned processes and hidden AppData execution using the `NormalizedTelemetryEvent` schemas.
