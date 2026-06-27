# Known Limitations

- **Legacy Windows Support:** Windows 7, 8, and Server 2012/2016 are unsupported due to missing modern ETW telemetry traces.
- **Network Providers:** Network telemetry relies entirely on ETW and currently lacks a deep packet inspection (DPI) driver or NDIS filter for advanced protocol termination.
- **Remote Administration:** Currently restricted to local IPC via Tauri. Remote Fleet Management requires future implementation of a Remote Communication Provider (e.g., gRPC / TLS).
