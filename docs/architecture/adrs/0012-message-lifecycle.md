# ADR 0012: Message Lifecycle and Contracts

## Status
Accepted

## Context
Messages flying across the bus must carry explicit metadata to support future distributed tracing, correlation, and replay mechanisms.

## Decision
All messages (Events and Commands) must wrap a standard `MessageMetadata` struct containing: `MessageId`, `CorrelationId`, `CausationId`, `Timestamp`, `ProducerId`, and `SchemaVersion`.

## Consequences
Every engine producing a message must generate this metadata. Correlation and causation IDs allow analysts to track exactly which ETW event triggered which Detection Alert, and which Alert triggered which Remediation Command.
