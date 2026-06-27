# ADR 0002: Process Cache Eviction and Lifecycle

## Status
Accepted

## Context
The Process Engine caches static `ProcessMetadata` (such as Image Path, Command Line, and initial Token privileges) to avoid repeatedly querying the OS for every telemetry event (e.g., every registry key access by a process). However, since processes are ephemeral, an unbounded cache will inevitably cause an out-of-memory (OOM) crash in long-running deployments.

## Decision
We will implement an explicit cache lifecycle with the following rules:

1. **Insertion Policy:** A process is inserted into the cache ONLY upon a verified `ProcessCreate` event or upon the first occurrence of an unknown PID (a "late discovery"). 
2. **Update Policy:** The cache acts as an optimization layer, NOT the source of truth. If a `ProcessSnapshot` observes new static metadata (e.g., a late-loaded environment variable), it may update the cache. However, the cache explicitly tolerates missing or stale entries.
3. **Expiration Policy:** 
   - A `ProcessExit` event triggers an immediate soft-tombstone of the cache entry.
   - If a new `ProcessCreate` event arrives with the same PID, the tombstoned entry is forcibly evicted and replaced.
4. **Cleanup Policy:** A periodic sweep (e.g., every 5 minutes) removes all tombstoned entries, and strictly evicts any active process that has not generated any telemetry in the last 24 hours (LRU/TTL fallback).

## Alternatives Considered
- **Strict OS Mirroring:** Periodically querying the OS (`EnumProcesses`) to keep the cache perfectly synchronized. 
  - *Rejected:* Introduces massive overhead and race conditions.

## Trade-offs
- *Pros:* Bounds memory strictly. Prevents PID collision data corruption.
- *Cons:* TTL fallbacks might evict completely dormant background processes, resulting in a slight latency penalty (cache miss OS query) when they eventually wake up.

## Consequences
The analyzer must gracefully handle cache misses and gracefully handle API failures (e.g., `ACCESS_DENIED`) when forced to query the OS directly on a miss.
