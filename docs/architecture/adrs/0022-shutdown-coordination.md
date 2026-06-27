# ADR 0022: Shutdown Coordination

## Status
Accepted

## Decision
Shutdown must occur in exact reverse dependency order. A service cannot be shut down until all services that depend on it have successfully exited or timed out.
