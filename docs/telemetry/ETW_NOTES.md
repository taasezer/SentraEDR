# SentraEDR ETW Notes

## Ingestion Pipeline
The ETW ingestion pipeline is strictly isolated into discrete stages to prevent cascading failures:

1. **Provider Management (`provider.rs`)**: Defines the GUIDs, Keywords, and Levels required.
2. **Session Management (`session.rs`)**: Manages the ETW session lifecycle (`StartTrace`, `EnableTraceEx2`). A failure here does not crash the application but prevents telemetry flow.
3. **Parsing (`parser.rs`)**: The blocking `ProcessTrace` C-callback runs on a dedicated OS thread. It translates the ephemeral `EVENT_RECORD` into an owned `RawEtwEvent` and pushes it into a bounded `mpsc` channel.
4. **Normalization (`normalizer.rs`)**: An async Tokio task reads `RawEtwEvent`s and translates them into a platform-agnostic `NormalizedTelemetryEvent`.

## Failure Boundaries
- A parser failure (e.g., malformed event) logs an error and increments the `parser_failures` metric but does NOT crash the parsing thread or stop the session.
- A normalization failure drops the event and increments `normalization_failures` without affecting other pipeline stages.
- A full queue drops the oldest/lowest-priority event, incrementing `queue_overflows`, ensuring the blocking OS thread is never paused.

## Target Providers
- **Process Creation:** `Microsoft-Windows-Kernel-Process` (`22FB2CD6-0E7B-422B-A0C7-2FAD1FD0E716`)
- **Network (Future):** `Microsoft-Windows-Kernel-Network`
- **PowerShell:** `Microsoft-Windows-PowerShell` (`A0C1853B-5C40-4B15-8766-3CF1C58F985A`)

## Event Ordering Guarantees
- **No Strict Sequentiality:** ETW does NOT guarantee perfectly sequential delivery of events across multiple CPU cores or tracing sessions. There can be timestamp jitter.
- **Correlation Strategy:** The downstream `engine-detection` MUST NOT assume strict ordering. Attack chains must be correlated using time-windows (e.g., Event B occurred within 5 seconds of Event A for the same PID) rather than expecting `Event A` to arrive precisely before `Event B` in the queue. Event IDs and unique process correlation strings are used to stitch states together.
