# Architecture Invariants

These invariants are permanent and unbreakable.

1. **Crate Ownership Rules:** Engines do not own persistence. Storage Providers do not own logic. The Runtime owns Composition.
2. **Dependency Direction:** All dependencies point INWARD toward abstractions (`core-*`). Concrete implementations (`infrastructure-*`) depend on core abstractions.
3. **Lifecycle Ownership:** `service-host` -> `RuntimeBuilder` -> `Supervisor`. No child thread outlives the Supervisor.
4. **Cancellation Ownership:** The `CancellationToken` hierarchy flows strictly top-down. Parent cancellation aborts all descendants.
5. **Error Propagation Rules:** Engines do NOT panic. Errors bubble up to the `Supervisor` which determines the Retry Policy.
6. **Telemetry Flow Rules:** Zero-copy parsing via `EventRecordParser`. Slices do not outlive the ETW session buffer.
7. **UI Isolation Rules:** The UI is a replaceable client. It must NEVER bypass the `EventBus`/`CommandBus`.
