# SentraEDR Performance Notes

Date: 2026-06-27
Phase: 18

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

Phase 12:

- IPC frames must enforce a maximum payload size before deserialization and must not allocate unbounded buffers.

Phase 13:

- IPC dispatch queues must remain bounded and report pressure through route stats.

Phase 14:

- IPC intake must count decode and dispatch failures without unbounded buffering.

Phase 15:

- IPC stream assembly must bound incomplete frame buffering and reject oversized prefixes early.

Phase 16:

- IPC pipeline composition must provide a low-overhead path from raw bytes to dispatched messages without adding unbounded buffering or excessive allocations per chunk.

Phase 17:

- Agent-side IPC service integration must preserve bounded dispatcher queues and expose aggregate counters for demo telemetry without streaming raw high-volume events.

Phase 18:

- UI live telemetry projection must store aggregate panel state and timeline entries only, not raw telemetry streams.

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

## Phase 12 Status

IPC frame validation now rejects payloads larger than 1 MiB before deserialization and rejects incomplete frames before parsing. The implementation performs in-memory serialization only; it introduces no named-pipe loop, socket, background scheduler, persistent store, unbounded channel, production telemetry stream, or benchmark claim.

## Phase 13 Status

IPC dispatch now routes validated messages into bounded per-category queues and records accepted, rejected, and dropped counts. The implementation introduces no named-pipe loop, socket, background scheduler, persistent store, unbounded channel, production telemetry stream, or benchmark claim.

## Phase 14 Status

IPC frame intake now composes frame decode and bounded dispatch while tracking accepted, decode-failed, and dispatch-failed frame counts. The implementation accepts complete frames only and introduces no stream buffer, named-pipe loop, socket, background scheduler, persistent store, unbounded channel, production telemetry stream, or benchmark claim.

## Phase 15 Status

IPC stream assembly now buffers incomplete frame bytes up to one maximum-sized frame and emits complete frames for downstream intake. Oversized prefixes are rejected before payload buffering. The implementation introduces no named-pipe loop, socket, background scheduler, persistent store, unbounded channel, production telemetry stream, or benchmark claim.

## Phase 16 Status

IPC pipeline composition now integrates the stream assembler, frame intake, and dispatcher into a single processing unit. The implementation validates that raw byte chunks are efficiently translated into dispatched IPC messages with minimal overhead and correct failure accounting. This phase remains strictly in-memory and introduces no named-pipe transport, async read loops, or unbounded buffering.

## Phase 17 Status

The agent IPC service skeleton now wraps the in-memory pipeline behind bounded configuration and records synthetic dry-run counters during observe-only startup. The integration does not add named-pipe transport, socket loops, persistent storage, background polling, unbounded channels, raw telemetry streaming, or benchmark claims.

## Phase 18 Status

The UI live telemetry projection stores a single latest panel and sorted timeline entries derived from aggregate snapshots. It does not add rendering loops, live IPC transport, persistent storage, background polling, unbounded channels, or raw event retention.
