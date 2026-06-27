# Disaster Recovery Review

## Analyzed Scenarios
- **Corrupted Storage:** `StorageProvider` initializes with `.wal` recovery. If unrecoverable, an empty database is generated and an `EventSink` Critical alert is emitted.
- **Corrupted Configuration:** Fall back to last-known-good configuration or compiled-in defaults.
- **Runtime Crash:** `CrashHandler` intercepts, emits stacktrace via `EventSink`, `service-host` propagates termination to Windows SCM for recovery policy (restart).
- **ETW Unavailable:** Supervisor transitions ETW provider to `Degraded` state. Exponential backoff is applied for reconnection.
- **Invalid Rules:** `RuleManifest` validation drops invalid rules during startup. The engine remains functional.
