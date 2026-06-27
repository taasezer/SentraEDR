# Administrator Guide

## Agent Management
The agent runs headlessly as `NT AUTHORITY\SYSTEM`. Management of rules, configurations, and health monitoring is achieved locally via the Tauri Desktop UI, which communicates with the backend Service Host exclusively via secure IPC over the `CommandBus`.

## UI Authorization
The Tauri Desktop UI requires local Administrator group privileges to open the named pipes facilitating communication with the engine. Standard users cannot read telemetry or manipulate rules.
