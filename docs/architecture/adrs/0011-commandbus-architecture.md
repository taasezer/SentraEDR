# ADR 0011: CommandBus Architecture

## Status
Accepted

## Context
While the EventBus declares "something happened," the CommandBus declares "do this work." Work requests cannot be silently dropped without notifying the caller.

## Decision
The CommandBus enforces Reliable Delivery. It utilizes point-to-point bounded queues (`mpsc`) with explicit backpressure (returning `TrySendError`) and Dead Letter Queues (DLQ) for commands that permanently fail. 

## Consequences
If the remediation engine is overloaded, the detection engine will receive an explicit backpressure error when attempting to send a remediation command, allowing the detection engine to decide whether to retry or degrade gracefully.
