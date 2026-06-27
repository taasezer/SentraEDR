# SentraEDR Memory Model

Date: 2026-06-27
Phase: 0

## Target

The initial agent target is less than 150 MB idle memory with minimal idle CPU. This is a design target for later benchmarks, not a Phase 0 runtime measurement.

## Allocation Principles

- Bounded queues in every production telemetry path.
- Compact event models.
- Avoid repeated string cloning in hot paths.
- Avoid JSON serialization in hot telemetry loops.
- Prefer binary framing for IPC and storage handoff.
- Aggregate repetitive low-value events during pressure.
- Keep heavyweight modules unloaded until needed.

## Queue Strategy

Each queue must define:

- capacity;
- priority handling;
- overflow behavior;
- metrics for depth, drops, and latency.

Default pressure behavior:

- Critical events are retained first.
- Low-priority repetitive events are dropped or aggregated first.
- Queue pressure is surfaced as telemetry health data.
- Detection findings include uncertainty when telemetry loss is known.

## Runtime Separation

ETW ingestion must not wait on registry, file, UI, SQLite, or network analysis work. Blocking or slow operations are routed to dedicated IO execution paths.

## Logging Discipline

Logs can create memory and disk pressure if uncontrolled. Logging rules:

- structured logs;
- rate limits for repeated events;
- no high-volume raw event dumps by default;
- debug tracing requires explicit mode;
- sensitive command-line data must be redacted where policy requires it.

## Measurement Plan

Later phases must add:

- idle memory measurement;
- event burst measurement;
- queue saturation tests;
- drop-rate tests;
- allocation checks for hot paths;
- database write pressure tests.

## Phase 0 Status

The memory model is a contract. No memory benchmark exists yet because no executable agent exists in Phase 0.
