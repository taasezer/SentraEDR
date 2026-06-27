# Frontend Security Review

## Attack Surface Mitigation
- **XSS Prevention:** Strict Content Security Policy (CSP) blocking `unsafe-inline` and `unsafe-eval`.
- **Tauri Command Validation:** The Rust `tauri-app` IPC layer validates all incoming structs (JSON payloads). Malformed data is rejected before hitting `ui-api-client`.
- **Serialization Attacks:** DTOs are strongly typed. Unknown fields in JSON payloads are rejected by `serde_json`.
- **Privilege Escalation:** The WebView runs in standard user context. It can only dispatch approved `CommandBus` actions. It cannot arbitrarily spawn elevated shells.
