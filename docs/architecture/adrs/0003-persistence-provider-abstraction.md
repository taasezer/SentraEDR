# ADR 0003: Persistence Provider Abstraction

## Status
Accepted

## Context
Persistence mechanisms span a vast array of Windows OS features (Registry, File System, WMI, COM, Tasks). Hardcoding the `analyzer` to directly query these OS APIs makes the engine impossible to test offline, difficult to mock, and violates the single-responsibility principle.

## Decision
We will introduce a `PersistenceProvider` trait within the `engine-persistence/source.rs` layer. Every persistence mechanism (Registry, WMI, FileSystem) must implement this trait. 

The trait enforces a unified contract:
- `query(identity)` -> `PersistenceSnapshot`
- `list_all()` -> `Vec<PersistenceSnapshot>`

The `analyzer` will hold a collection of `Box<dyn PersistenceProvider>` instances. When an event arrives, the analyzer delegates to the correct provider.

## Alternatives Considered
- **Direct Win32/COM Calls in Analyzer:** Fastest execution, but impossible to test via CI/CD without full Windows environments.
- **Provider-Specific Engines:** Splitting `engine-registry`, `engine-wmi`. Rejected because correlation of persistence often spans multiple domains, and maintaining 10 engines for persistence is overkill.

## Trade-offs
- *Pros:* Fully mockable. Easy to add new providers (e.g., IFEO) without touching core logic.
- *Cons:* Dynamic dispatch overhead (`dyn`) when routing events to the correct provider. Given persistence events are relatively low frequency compared to ETW Process/Network events, this trade-off is highly acceptable.

## Consequences
The engine must strictly limit all provider implementations to **read-only** operations. No provider is permitted to modify state on the host system.
