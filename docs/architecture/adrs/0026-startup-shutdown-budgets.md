# ADR 0026: Startup and Shutdown Budgets

## Status
Accepted

## Decision
Every ComponentManifest declares an `estimated_startup_time_ms`. The Orchestrator treats this as a hard budget. If an engine's `initialize()` or `start()` blocks the reactor beyond this budget, the Orchestrator panics the runtime. During shutdown, exceeding the budget results in a forced cancellation cascade and a structured diagnostic report.
