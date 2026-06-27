# SentraEDR Architecture Health Report

## 1. Dependency Graph Status
The project employs a clean, uni-directional dependency graph:
- `shared-models` forms the foundation.
- `engine-etw`, `engine-process`, `engine-network`, and `engine-persistence` consume `shared-models` and emit normalized snapshots without depending on each other.
- `engine-detection` solely consumes snapshots from the other engines. It makes absolutely zero OS API calls.

## 2. Crate Isolation Review
Zero circular dependencies exist. Engines interact strictly through immutable message-passing of `NormalizedTelemetryEvent` and domain-specific snapshots.

## 3. Coupling Analysis
- **Telemetry Coupling:** All engines are decoupled from specific host technologies (e.g., `PersistenceProvider` and `NetworkProvider` traits prevent locking the EDR to ETW or Npcap).
- **Rule Coupling:** The Detection Engine decoupled Risk scoring from Confidence scoring, and separated the Rule definitions from the underlying Correlation State logic.

## 4. Memory Budget Review
- Memory usage is tightly bounded.
- The `DnsCache` uses LRU limits.
- The `ProcessCache` uses explicit tombstones and TTL eviction.
- The `CorrelationState` strictly purges historical events based on the maximum TTL defined by the active `Rule` set, alongside a hard cap of 50,000 events. Memory exhaustion via telemetry floods is structurally mitigated.

## 5. Performance Objectives
- Thread contention has been eradicated from the hot paths.
- Asynchronous DNS resolution prevents network latency from blocking process correlations.
- Atomic metrics ensure zero-overhead observability.

## 6. Technical Debt Assessment
- **Debt:** Currently utilizing mock providers for Network and Persistence. These will need concrete kernel/ETW implementations.
- **Debt:** `shared-models` needs to establish the unified `NormalizedTelemetryEvent` pipeline routing (e.g. mpsc channels) across threads.

## 7. Architectural Risks
- Handling extremely high-volume ETW bursts (e.g. 100,000 events/sec) might overwhelm the single-threaded `CorrelationState` deque. If this occurs, we will need to partition the correlation state by ProcessIdentity or CPU core.

## 8. Readiness for Remediation Engine
The pipeline terminates cleanly in immutable `Alert` objects. The architecture is fully prepared to introduce `engine-remediation` which will consume these Alerts and execute blocking logic via a separated OS interaction layer.
