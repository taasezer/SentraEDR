# ADR 0021: Health Coordinator

## Status
Accepted

## Decision
The `HealthCoordinator` aggregates asynchronous health checks (`Startup Health`, `Runtime Health`, `Liveness`, `Readiness`) from all active Services. If a core infrastructure service fails liveness, the coordinator triggers a platform graceful shutdown.
