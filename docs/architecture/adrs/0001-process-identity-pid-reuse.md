# ADR 0001: Process Identity and PID Reuse Handling

## Status
Accepted

## Context
Operating systems recycle Process IDs (PIDs) aggressively. During long-running tracing sessions, relying solely on a `u32` PID to uniquely identify a process leads to collisions, false positives in correlation, and memory corruption inside caches. The Process Engine needs a robust way to establish unique identity across the entire telemetry pipeline.

## Decision
We will adopt a composite `ProcessIdentity` model that globally uniquely identifies an executing process instance. 

The identity is defined as:
- **`pid`**: The OS-level Process ID.
- **`creation_time`**: The precise epoch timestamp of the process creation event.
- **`event_id`**: The UUID of the `NormalizedTelemetryEvent` that originally announced the creation of this process.

## Alternatives Considered
- **PID + Sequence Number**: Relies on internal counters which are easily desynchronized if the telemetry source drops a process creation event.
- **ETW ProcessKey (Windows 10+)**: Highly reliable, but entirely couples the engine to ETW, breaking our `TelemetrySource` abstraction which mandates support for Sysmon or Linux down the line.

## Trade-offs
- *Pros:* Fully decoupled from ETW specifics. Guarantees uniqueness across reboots and PID recycling loops.
- *Cons:* Requires downstream engines (like Detection) to perform slightly more complex identity matching rather than simple `HashMap<u32, State>` lookups.

## Consequences
All internal caches (`ProcessCache`) and downstream engines must use `ProcessIdentity` as the primary key. If a telemetry event arrives with a PID but no known `creation_time` (e.g., an orphan event before the creation event was processed), the engine must synthesize a temporary identity and attempt to reconcile it later, or treat it as an un-cached point-in-time snapshot.
