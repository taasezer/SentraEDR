# Enterprise Observability Guide

## Core Metrics
- **Agent Health:** Uptime, Thread Count, RSS Memory (Target: < 50MB).
- **ETW Health:** Throughput (events/sec), buffer saturation, drop rate.
- **Detection Latency:** p95 and p99 rule execution latency tracked via `RuleProfiler`.

## Logs
- **Structured Logging:** All internal Agent traces are JSON-formatted and emitted to the `EventSink` (Windows Event Log).
- **Security Events:** Detections are dispatched into isolated event channels ensuring they never intermingle with diagnostic logs.
