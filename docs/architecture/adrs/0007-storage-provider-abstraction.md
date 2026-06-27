# ADR 0007: Storage Provider Abstraction

## Status
Accepted

## Context
SentraEDR requires persistent storage for immutable telemetry, audit, and alert events. Directly embedding SQLite or PostgreSQL into the core engine violates our platform-agnostic, decoupled architecture and prevents future integration with central logging systems or distributed event buses.

## Decision
All storage operations must funnel through the generic `StorageProvider` and `EventRepository` abstractions. Engines are strictly forbidden from executing raw SQL queries or depending on database-specific crates (like `rusqlite`). 

## Alternatives Considered
- **Direct Database Dependencies:** Binds the entire codebase to a specific ORM or DB connector. Hard to test offline.
- **Microservices:** Emitting events over gRPC. Overkill for local endpoint storage in a single agent.

## Trade-offs
- *Pros:* Easy to mock for tests. Easy to swap SQLite for DuckDB or a Remote gRPC Storage sink.
- *Cons:* Requires abstracting schema migrations away from database-specific ORM tools.

## Consequences
The `infrastructure-storage` crate will maintain its own `InMemoryStorageProvider` for initial validation, and will expose trait bounds for future specific implementations.
