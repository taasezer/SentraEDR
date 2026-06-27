# SentraEDR Performance Notes

Date: 2026-06-27
Phase: 11

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

Phase 11:

- Quality gates must remain bounded to compile, lint, test, architecture validation, and observe-only dry-run checks.

## Phase 0 Status

Performance is specified as design targets. Measurements begin when executable components exist.

## Phase 2 Status

The first executable telemetry path is available through synthetic process ETW records. Validation covers bounded queue delivery, queue pressure, dropped-event accounting, and component health degradation. No real Windows ETW burst benchmark or memory measurement has been claimed yet.

## Phase 3 Status

Synthetic process analysis validates small in-memory state updates and deterministic string matching for initial process signals. The implementation introduces no regex engine, no unbounded channels, no persistent store, and no real process enumeration. Retention limits and memory pressure policies remain future work before high-volume process telemetry is claimed.

## Phase 4 Status

Synthetic persistence analysis validates metadata extraction and deterministic string matching for initial persistence signals. The implementation introduces no regex engine, no Windows API calls, no filesystem scanning, no unbounded channels, and no persistent store. High-volume registry or Windows Event Log ingestion benchmarks remain future work.

## Phase 5 Status

Synthetic network analysis validates metadata extraction, small destination history updates, and deterministic string matching. The implementation introduces no packet capture, DNS resolver, socket operations, firewall modification, persistent store, regex engine, or unbounded channel. High-volume connection telemetry and beacon statistics remain future work.

## Phase 6 Status

Synthetic detection analysis validates small-batch signal scoring and family diversity correlation. The implementation introduces no rule interpreter, persistent store, unbounded channel, ML model, or remediation path. Large correlation windows and false-positive suppression benchmarks remain future work.

## Phase 7 Status

Synthetic remediation analysis validates deterministic policy checks, small in-memory plan construction, and audit metadata generation. The implementation introduces no blocking OS calls, filesystem mutation, registry access, firewall modification, process control, persistent store, or unbounded channel. Real quarantine throughput, rollback overhead, and audit persistence benchmarks remain future work.

## Phase 8 Status

Synthetic memory analysis validates deterministic metadata parsing and signal mapping. The implementation introduces no process memory reads, dumps, handle enumeration, driver calls, blocking Windows memory APIs, persistent store, regex engine, or unbounded channel. Real high-volume memory telemetry, process access overhead, and retention benchmarks remain future work.

## Phase 9 Status

Synthetic dashboard state construction validates small in-memory alert sorting, risk counting, timeline ordering, and pending action aggregation. The implementation introduces no browser runtime, IPC client, persistent store, unbounded channel, polling loop, rendering benchmark, or high-frequency UI stream.

## Phase 10 Status

Synthetic testing infrastructure validates scenario catalog construction, safety gating, phase coverage counting, and MITRE tag aggregation. The implementation introduces no VM orchestration, command runner, malware execution, live IPC fuzzing, persistent store, unbounded channel, or host mutation.

## Phase 11 Status

CI quality gates validate the workspace with deterministic build, lint, test, architecture, and observe-only dry-run commands. The implementation introduces a local command runner and GitHub Actions workflow, but no production telemetry load, benchmark claim, VM orchestration, deployment job, release signing, malware execution, remediation execution, persistent store, unbounded channel, or host mutation.
