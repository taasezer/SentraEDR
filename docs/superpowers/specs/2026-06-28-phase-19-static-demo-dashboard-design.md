# Phase 19: Static Demo Dashboard Design

## Goal
Produce an openable, demo-ready SentraEDR dashboard HTML artifact from the existing `sentra-ui` state model.

## Scope
- Add a deterministic HTML renderer to `sentra-ui`.
- Add a Rust example that builds a synthetic dashboard and writes `target/sentra-demo-dashboard.html`.
- Keep the demo static and offline.
- Keep `sentra-ui` independent from `sentra-agent`, `shared-ipc`, and engine crates.
- Do not add a web server, JavaScript runtime, named-pipe transport, command channel, user approval execution, remediation execution, or live Windows integration.

## Architecture
`sentra-ui` remains the owner of display projection. The new renderer receives `DashboardState` and returns a complete HTML document. The example binary owns synthetic demo data only and writes an HTML file under `target/`.

New units:

- `crates/sentra-ui/src/demo_html.rs`
  - Renders `DashboardState` into static HTML.
  - Escapes dynamic text fields.
  - Presents telemetry counters, risk summary, alerts, pending actions, and timeline.
- `crates/sentra-ui/examples/demo_dashboard.rs`
  - Creates a synthetic dashboard state.
  - Applies a live telemetry snapshot.
  - Adds a pending action card.
  - Writes `target/sentra-demo-dashboard.html`.

## UI Direction
The dashboard should feel like a SOC/operator surface: dense, quiet, readable, and demo-friendly. It should use restrained contrast, metric panels, status indicators, timeline rows, and alert/action lists rather than a marketing landing page.

## Validation
- Renderer tests verify key sections and telemetry values appear.
- Renderer tests verify dynamic text escaping.
- Example build/run generates the HTML file.
- Full workspace quality gates remain green.
