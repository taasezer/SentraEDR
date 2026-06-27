# ADR 0010: EventBus Architecture

## Status
Accepted

## Context
SentraEDR needs to distribute thousands of events per second across multiple engines without coupling the producers to the consumers.

## Decision
We introduce `core-eventbus` utilizing a publish-subscribe model. The EventBus must use strongly typed generic interfaces (`subscribe<T>()`) to enforce compile-time safety and eliminate runtime string-routing bugs.

## Consequences
Producers will fire and forget events. Consumers must define the specific Rust types they are subscribing to.
