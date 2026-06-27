# ADR 0013: Delivery Guarantees

## Status
Accepted

## Decision
The EventBus implements **Best-Effort Delivery**. It guarantees that an event is handed to the underlying channel, but does not guarantee the subscriber will successfully process it before the channel overflows or the agent shuts down.
The CommandBus implements **Reliable Delivery**. Commands that fail execution are routed to a Dead Letter Queue (DLQ).
