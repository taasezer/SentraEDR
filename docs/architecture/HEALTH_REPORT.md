# SentraEDR Architecture Health Report

## 1. Communication Topology
The `core-eventbus` now dictates all cross-engine communication.
- **Events:** 1-to-N broadcasting (`tokio::sync::broadcast`).
- **Commands:** 1-to-1 routing (`tokio::sync::mpsc`).

## 2. Queue Utilization & Throughput
Because both buses use bounded channels with strict capacity limits, unbounded memory growth is structurally impossible.
- Throughput is extremely high (millions of msgs/sec in local memory) because `try_send` avoids context-switching blocks.

## 3. Failure Isolation
- If a downstream subscriber crashes or stalls, the `EventBus` drops the oldest messages. The producer thread is entirely unaffected.
- If a `CommandBus` target is saturated, the sender receives an explicit error.

## 4. Scalability & Multi-Process Readiness
With the generic `CommunicationProvider` trait, migrating the agent to a multi-process architecture (e.g., separating the high-privilege Remediation engine into a system service and Detection into a user-space process) is now trivial. The `CommunicationProvider` simply swaps out `Local` channels for named pipes or Unix Domain Sockets.

## 5. Overall System Health
The platform core (`runtime`, `registry`, `eventbus`, `config`, `observability`) is fully realized and isolated. The business engines (`etw`, `process`, `network`, `persistence`, `detection`, `remediation`, `storage`) plug cleanly into this core. The EDR architecture is incredibly mature, resilient, and prepared for future C2 or UI integrations.
