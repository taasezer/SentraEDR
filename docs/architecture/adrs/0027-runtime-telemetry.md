# ADR 0027: Runtime Telemetry

## Status
Accepted

## Decision
The `Runtime` orchestrator publishes its own lifecycle events (`RuntimeStarted`, `ServiceStarted`, `ServiceStopped`, `RuntimeStopping`, `RuntimeStopped`) directly onto the `EventBus`. This turns the Runtime into an observable component just like any business engine.
