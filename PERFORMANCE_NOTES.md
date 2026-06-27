# SentraEDR Performance Notes

Date: 2026-06-27
Phase: 3

## Performance Goals

- Agent idle memory target: less than 150 MB.
- Idle CPU: minimal and event-driven.
- Telemetry pipeline: bounded and backpressure-aware.
- UI: isolated from detection and ingestion.
- Database: SQLite only for local lightweight persistence.

## High-Risk Performance Areas

ETW ingestion:

- Event bursts can overwhelm downstream consumers.
- Parser work must stay small and predictable.

Detection correlation:

- Unbounded process graphs or long retention windows can grow memory.
- Correlation windows must expire.

Network analysis:

- Packet-level capture can be expensive.
- Early phases should prefer connection metadata before full packet parsing.

Logging:

- Raw telemetry logs can dominate disk and memory.
- Debug modes must be explicit.

UI updates:

- Streaming every event to UI can create unnecessary load.
- UI should receive summaries and alerts first.

## Planned Metrics

- queue depth;
- enqueue latency;
- dequeue latency;
- event drop count;
- event aggregation count;
- memory usage by process;
- CPU idle and burst usage;
- SQLite write latency;
- alert generation latency.

## Phase Gates

Phase 1:

- Compile and dependency checks.
- No runtime performance claims.

Phase 2:

- ETW event ingestion load test for a narrow process-event path.
- Queue pressure behavior verified.

Phase 3-6:

- Analyzer and scoring latency checks.
- Correlation memory retention checks.

Phase 7:

- Remediation audit and rollback overhead checks.

Phase 9:

- UI update throttling and non-blocking agent behavior checks.

## Phase 0 Status

Performance is specified as design targets. Measurements begin when executable components exist.

## Phase 2 Status

The first executable telemetry path is available through synthetic process ETW records. Validation covers bounded queue delivery, queue pressure, dropped-event accounting, and component health degradation. No real Windows ETW burst benchmark or memory measurement has been claimed yet.

## Phase 3 Status

Synthetic process analysis validates small in-memory state updates and deterministic string matching for initial process signals. The implementation introduces no regex engine, no unbounded channels, no persistent store, and no real process enumeration. Retention limits and memory pressure policies remain future work before high-volume process telemetry is claimed.
