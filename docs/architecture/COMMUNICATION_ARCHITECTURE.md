# Communication Architecture

SentraEDR utilizes a sophisticated, type-safe Communication layer strictly separating Commands from Events.

## EventBus vs CommandBus
- **EventBus:** Best-effort, 1-to-N broadcasting. Bounded channels drop old messages to protect memory. Slow subscribers lag independently and receive explicit errors when they fall too far behind.
- **CommandBus:** Reliable, 1-to-1 routing. Backpressure is explicitly returned to the caller (`TrySendError`). Failures are sent to a Dead Letter Queue (DLQ).

## Message Contracts
All messages conform to a strict metadata contract including `CorrelationId` and `CausationId` to establish an unbroken lineage from an OS event to a remediation action.

## Capability Integration
Transport providers (Local, Named Pipes, gRPC) implement the `CommunicationProvider` trait and register into `core-registry`. The `BootstrapOrchestrator` wires them up on boot.
