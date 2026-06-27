# UI State Architecture

## State Ownership
- The UI must NOT maintain a parallel global cache of the EDR database.
- State is owned by specific View Models (e.g., `DashboardState`) and scoped to the active component lifecycle.

## Synchronization Model
- State is derived strictly from `EventBus` pushed `EventMessage` payloads.
- If a component unmounts, its EventBus subscription is explicitly dropped. Stale caches are immediately collected.

## Persistence Strategy
- The UI persists absolutely nothing locally. Configuration edits are dispatched as `UpdateConfigurationCommand` and re-read from the backend upon successful commit.
