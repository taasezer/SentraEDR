# ADR 0018: Runtime Builder

## Status
Accepted

## Decision
We enforce a `RuntimeBuilder` pattern instead of a Service Locator. The Builder accepts injected abstractions (EventBus, CommunicationProviders) and builds the final `Runtime` instance, completely eliminating global singletons.
