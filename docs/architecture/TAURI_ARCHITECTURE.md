# Tauri Architecture

The UI is packaged into four crates to enforce domain-driven design on the client side:
1. `ui-models`: Standalone UI state objects (e.g., `AlertView`). These do NOT implement backend `EventMessage` traits directly, enforcing a mapping boundary.
2. `ui-api-client`: The sole orchestrator of `core-eventbus`. Mocks are injected via Dependency Inversion.
3. `ui-core`: Owns explicit UI lifecycles (e.g., `DashboardState`).
4. `tauri-app`: Binds `ui-core` state logic to the Tauri IPC window context.
