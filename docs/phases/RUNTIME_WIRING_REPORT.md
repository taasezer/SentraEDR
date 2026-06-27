# Runtime Wiring Report

## Progressive Topology
1. `Telemetry` -> `EventBus` (Publishes raw ETW)
2. `Process`, `Network`, `Persistence` -> `EventBus` (Enriches and normalized OS events)
3. `Detection` -> `CommandBus` (Receives normalized events, publishes Remediation Commands)
4. `Remediation` -> `EventBus` (Receives Commands, outputs Audit Events)
5. `Storage` -> (Consumes all events for archival via `CommunicationProvider`)

## Instantiated Components
All engines correctly registered via `CapabilityRegistry` without explicit static coupling. Dependency injection succeeded across all boundaries.
