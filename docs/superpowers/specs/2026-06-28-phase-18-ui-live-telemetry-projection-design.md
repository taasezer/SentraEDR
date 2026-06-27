# Phase 18: UI Live Telemetry Projection Design

## Goal
Add a demo-ready live telemetry projection layer to `sentra-ui` so the dashboard can represent agent health, telemetry counters, IPC health, and recent update timing without depending on `sentra-agent` or live IPC transport.

## Scope
- Add UI-facing telemetry snapshot types inside `sentra-ui`.
- Keep `sentra-ui` dependent only on `shared-models`.
- Extend `DashboardState` with a telemetry panel and telemetry update timeline entries.
- Use synthetic tests only.
- Do not build a browser renderer, named-pipe client, local server, remediation execution path, or Windows integration in this phase.

## Architecture
`sentra-ui` remains a projection crate. It accepts already-normalized demo data through value types and turns that data into dashboard state. Future transport code can produce the same snapshot values without forcing UI code to import agent, IPC, or engine internals.

New responsibility boundaries:

- `crates/sentra-ui/src/live_telemetry.rs`
  - Owns `LiveTelemetrySnapshot`, `LiveTelemetryCounters`, `IpcTelemetryHealth`, and `LiveTelemetryPanel`.
  - Converts raw demo counters into UI-ready status fields.
- `crates/sentra-ui/src/dashboard.rs`
  - Stores the current `LiveTelemetryPanel`.
  - Adds telemetry update timeline entries.
- `crates/sentra-ui/src/timeline.rs`
  - Adds a `TelemetryUpdated` timeline kind.

## Data Model
`LiveTelemetrySnapshot` captures a single observe-only update:

- `observed_at`
- `agent_status`
- telemetry counters:
  - received
  - normalized
  - dropped
  - process signals
  - persistence signals
  - network signals
  - memory signals
  - detection alerts
- IPC health:
  - enabled
  - dispatcher capacity
  - frames accepted
  - failed frames
- highest priority seen in the update

`LiveTelemetryPanel` is the display-ready projection:

- agent status
- highest priority
- total telemetry received
- normalized event count
- dropped event count
- total behavioral signals
- detection alert count
- IPC enabled/disabled
- IPC frames accepted
- IPC failed frame count
- last updated timestamp

## Behavior
- A new `DashboardState::apply_live_telemetry(snapshot)` method replaces the current telemetry panel with the latest snapshot.
- Applying telemetry adds a `TelemetryUpdated` timeline entry.
- Timeline sorting keeps telemetry, alert, and action entries ordered by timestamp.
- Existing alert and action behavior remains unchanged.

## Security And Safety
- UI projection does not approve or execute remediation.
- No command channel is introduced.
- No live IPC transport is opened.
- No raw telemetry stream is persisted.
- All values are inert display data.

## Validation
- Add tests for live telemetry projection totals.
- Add tests that applying telemetry updates dashboard panel fields.
- Add tests that telemetry updates appear in timeline order with existing alert entries.
- Run full quality gates after implementation.
