# Phase 18 Report: UI Live Telemetry Projection

## Summary
Phase 18 moves the project closer to a demo dashboard by adding UI-side live telemetry projection state. The implementation lets `sentra-ui` represent agent health, aggregate telemetry counters, detection alert counts, and IPC health counters without importing agent, IPC, or engine crates.

## Implemented Changes
- Added `crates/sentra-ui/src/live_telemetry.rs`.
- Added `LiveTelemetryCounters` for aggregate telemetry and signal counts.
- Added `IpcTelemetryHealth` for IPC demo health counters.
- Added `LiveTelemetrySnapshot` as the input value for live/demo updates.
- Added `LiveTelemetryPanel` as the display-ready dashboard projection.
- Exported live telemetry types from `sentra-ui`.
- Added `DashboardState.telemetry`.
- Added `DashboardState::apply_live_telemetry`.
- Added `TimelineKind::TelemetryUpdated`.
- Added tests for snapshot projection, dashboard panel updates, summary stability, and timeline ordering.

## Security Notes
- This phase adds inert display/projection data only.
- No named-pipe client, browser renderer, command authorization, remediation execution, host mutation, malware execution, VM orchestration, deployment, or signing was added.
- UI state still cannot approve or execute remediation.

## Demo Impact
The UI model can now show whether the agent is healthy, how many telemetry events were received/normalized/dropped, how many behavioral signals were observed, how many alerts exist, and whether IPC is accepting frames. This gives the next phase a clean route toward an actual demo view or transport adapter.
