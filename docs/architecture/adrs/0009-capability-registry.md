# ADR 0009: Capability Registry & Orchestration

## Status
Accepted

## Context
As the engine scales with numerous rules, telemetry providers, and remediation actions, discovering and managing their lifecycles (Initialization, Dependency Checks, Shutdown) becomes chaotic. The Remediation engine's localized `ActionRegistry` is insufficient for system-wide orchestration.

## Decision
A new infrastructure crate, `core-registry`, will host a unified `CapabilityRegistry`. Providers and engines register themselves using strongly typed identifiers. A dedicated `BootstrapOrchestrator` will use this registry to resolve dependencies, initialize providers, monitor health, and handle graceful shutdowns.

## Alternatives Considered
- **Dependency Injection Frameworks:** Often rely heavily on macros and implicit magic, making debugging difficult.
- **Global Mutex singletons:** Creates massive contention across the agent.

## Trade-offs
- *Pros:* Clean, observable, strict initialization ordering. Prepares the architecture for a future EventBus.
- *Cons:* Requires a central crate that everything registers into at startup.

## Consequences
All future plugins and components must implement standard capability metadata and lifecycle traits to be recognized by the orchestrator.
