# Phase 3: Process Monitoring Engine Report

## Completed Work
- **ADRs Created:** 
  - `0001-process-identity-pid-reuse.md`: Defined the global composite key (`ProcessIdentity` mapping PID + CreationTime + EventID) to eliminate PID reuse corruption.
  - `0002-process-cache-eviction.md`: Explicitly formalized cache insertions (on process creation/discovery) and eviction rules (on process exit or TTL timeouts).
- **Process Engine Implementation (`engine-process`):**
  - Designed strictly around the new Engine Contract (`source`, `analyzer`, `models`, `metrics`, `errors`, `tests`).
  - Added strict separation of `ProcessMetadata` (immutable static attributes) and `ProcessStateChange` (dynamic transitions).
  - Enforced pure immutability of the incoming `NormalizedTelemetryEvent`s inside `analyzer.rs`.
  - Implemented `ProcessCache` which acts as an optimization layer, explicitly tolerating misses.
  - Enforced strict RAII semantics on Windows HANDLEs inside `source.rs` (`SafeHandle`), guaranteeing automatic resource cleanup.
  - Handled `ACCESS_DENIED` inside `source.rs` elegantly by mapping it to a structured `ProcessEngineError::AccessDenied` error instead of panicking.
- **Testing:**
  - Implemented synthetic tests verifying cache hits correctly retrieve static metadata while preserving original event immutability.

## Architectural Enforcement
- The `engine-process` crate has exactly zero dependencies on `engine-etw`. 
- The engine does zero detection scoring. It acts purely as a deterministic observer bridging raw telemetry streams with OS token states.

## Next Phase (Phase 4: Persistence Monitoring Engine / SQLite Storage)
- We will likely transition to tracking auto-start execution points (ASEP), registry modifications, and setting up the local storage layer using `rusqlite`.
