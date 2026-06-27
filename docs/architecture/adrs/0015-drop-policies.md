# ADR 0015: Drop Policies and Priorities

## Status
Accepted

## Decision
The EventBus explicitly implements Drop Policies to protect memory. If the `tokio::sync::broadcast` channel hits its maximum capacity, the oldest unread messages are silently dropped. To mitigate dropping critical security alerts, the EventBus supports Priority Queues. Lower-priority events (e.g., standard telemetry) are dropped before higher-priority events (e.g., Detection Alerts).
