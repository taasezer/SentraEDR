# ADR 0017: Service Lifecycle Contract

## Status
Accepted

## Decision
All long-lived components must implement the `Service` trait: `initialize()`, `start(CancellationToken)`, `stop()`, and `shutdown()`. The `start` function takes a cloned Tokio cancellation token to enforce a unified graceful shutdown hierarchy.
