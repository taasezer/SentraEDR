# Tauri Security Review

## IPC Audit
The frontend can only dispatch registered commands via `invoke()`. Unknown commands are rejected instantly.

## Dependency Assessment
The `tauri-app` crate utilizes zero external FFI dependencies outside of the Tauri ecosystem. 
Filesystem mapping is explicitly denied, preventing directory traversal attacks originating from the WebView.
