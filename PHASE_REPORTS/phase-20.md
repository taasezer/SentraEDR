# Phase 20 Report: Agent Demo Telemetry Snapshot + Dashboard Visual Polish

## Summary
Phase 20 bridges the agent dry-run pipeline output to the UI dashboard via a shared DTO, and significantly enhances the dashboard's visual quality. After this phase, the demo dashboard renders data derived from the agent's actual synthetic dry-run metric schema rather than hand-coded values.

## Phase 20A: Agent Demo Telemetry Snapshot Adapter

### Implemented Changes

- **shared-models**: Added `demo_snapshot.rs` with `DemoTelemetrySnapshot` DTO.
  - Lightweight struct with only primitive fields — no engine internals leak into shared-models.
  - Includes `empty()` constructor and `total_behavioral_signals()` helper.
  - 3 unit tests: zero-state, behavioral sum, clone+eq.

- **sentra-agent**: Added `snapshot_builder.rs` with `build_demo_snapshot()`.
  - Runs all 7 synthetic dry-run pipelines and collects results into a single `DemoTelemetrySnapshot`.
  - 6 unit tests validating ETW, behavioral signals, detection, IPC, remediation counters.

- **sentra-agent**: Refactored `main.rs` to use `build_demo_snapshot()`.
  - Eliminated duplicated dry-run calls; main now calls the builder once.
  - Tracing log fields sourced from the unified snapshot.

- **sentra-ui**: Added `from_demo_snapshot()` on `LiveTelemetrySnapshot`.
  - Converts the shared DTO into the UI's existing projection model.
  - Derives `highest_priority` from detection alert count.
  - 6 unit tests covering ETW mapping, behavioral signals, IPC, and priority derivation.

- **sentra-ui**: Updated `demo_dashboard.rs` example.
  - Now constructs a `DemoTelemetrySnapshot` and uses `from_demo_snapshot()`.
  - Added more diverse demo data: Critical, High, Medium, Low alerts; two pending actions.

### Architecture Compliance
- `sentra-ui` still only depends on `shared-models` — no agent/engine imports.
- `DemoTelemetrySnapshot` contains only primitive types.
- No new external dependencies added.

## Phase 20B: Dashboard Visual Polish

### Implemented Changes

- **Dark mode** — `#0f1117` background with glassmorphism card styling.
- **Google Fonts** — Inter via CDN link for modern typography.
- **Gradient title** — SentraEDR title with blue-purple gradient.
- **Risk-colored severity badges** — Tag classes for Critical (red), High (amber), Medium (blue), Low (green).
- **Telemetry progress bars** — Visual bars showing normalized/received ratio, drop rate, signal fill.
- **Status indicator pulse** — Animated green dot for Healthy, amber for Degraded.
- **Timeline color coding** — Left border per event kind: Alert (red), Action (amber), Telemetry (blue).
- **Pending action step details** — Each action shows its constituent steps as styled tags.
- **Generated timestamp footer** — Footer showing brand and generation timestamp.
- **Micro-animations** — Staggered `fadeIn` animation on card panels via CSS `@keyframes`.
- **Responsive layout** — Improved breakpoints at 860px and 520px.
- Added `generated_at: Timestamp` field to `DashboardState`.

## Security Notes
- The demo remains static HTML only.
- No web server, JavaScript command channel, named-pipe client, remediation execution, host mutation, malware execution, VM orchestration, deployment, or signing was added.
- All UI-rendered text is HTML-escaped.
- Observe-only principle maintained throughout.

## Demo Impact
The project now has a visually polished dark-mode dashboard that renders telemetry data through the same DTO schema the agent produces from its dry-run pipeline. The `DemoTelemetrySnapshot` DTO creates a clean integration point for future live telemetry without architectural violations.
