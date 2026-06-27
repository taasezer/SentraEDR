# SentraEDR Telemetry Model

## ETW as Source of Truth
The primary source of telemetry is Event Tracing for Windows (ETW). This provides a reliable, asynchronous, and high-performance mechanism for tracking system events.

## Monitored Providers
- **Process Provider:** Process creation and termination. Tracks command lines, parent PIDs, and execution paths.
- **Image Load Provider:** Tracks DLLs loaded into processes, enabling detection of unexpected or malicious library injections.
- **Registry Provider:** Monitors modifications to Run keys, service configurations, and other persistence mechanisms.
- **Network Provider (or Sysmon):** Outbound connections, DNS resolutions.
- **PowerShell Provider:** Tracks script block logging to detect obfuscated or malicious PowerShell execution.

## Telemetry Pipeline
1. **Ingestion:** `engine-etw` subscribes to ETW providers using an isolated Tokio runtime.
2. **Normalization:** Raw events are parsed into a standardized `NormalizedTelemetryEvent` schema defined in `shared-models`.
3. **Routing:** Events are routed via bounded channels (with backpressure strategies) to specialized engines (`engine-process`, `engine-network`, `engine-persistence`).
4. **Analysis:** The specialized engines enrich and filter the events, passing findings to `engine-detection`.

## Event Dropping Policy
Under extreme load, the system drops events based on priority:
- LOW (normal system noise) dropped first.
- MEDIUM (process metadata updates).
- HIGH (suspicious execution chains).
- CRITICAL (process injection, persistence creation) never dropped if possible.
