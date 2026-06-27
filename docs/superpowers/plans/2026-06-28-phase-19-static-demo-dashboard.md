# Phase 19 Static Demo Dashboard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Generate an openable static HTML demo dashboard from `sentra-ui` state.

**Architecture:** Add a renderer module to `sentra-ui` that converts `DashboardState` into a complete HTML document. Add a crate-local example that creates synthetic demo state and writes `target/sentra-demo-dashboard.html`; no transport, server, JavaScript runtime, or remediation execution is introduced.

**Tech Stack:** Rust workspace, `sentra-ui`, `shared-models`, static HTML/CSS, TDD tests, existing PowerShell quality gates.

---

### Task 1: HTML Renderer

**Files:**
- Create: `crates/sentra-ui/src/demo_html.rs`
- Modify: `crates/sentra-ui/src/lib.rs`
- Test: `crates/sentra-ui/tests/demo_html.rs`

- [x] **Step 1: Write failing renderer tests**

Create `crates/sentra-ui/tests/demo_html.rs` with tests that:

- build a synthetic dashboard;
- call `render_dashboard_html(&dashboard)`;
- assert the HTML contains SentraEDR demo sections and telemetry values;
- assert dynamic text is HTML-escaped.

- [x] **Step 2: Run renderer tests**

Run: `cargo test -p sentra-ui --test demo_html`

Expected: FAIL because `render_dashboard_html` is not exported.

- [x] **Step 3: Implement renderer**

Create `crates/sentra-ui/src/demo_html.rs` with:

- `pub fn render_dashboard_html(dashboard: &DashboardState) -> String`;
- a small `escape_html` helper;
- helper functions for status, priority, timeline kind, risk labels, and action labels.

- [x] **Step 4: Export renderer**

Modify `crates/sentra-ui/src/lib.rs` with `pub mod demo_html;` and `pub use demo_html::render_dashboard_html;`.

- [x] **Step 5: Re-run renderer tests**

Run: `cargo test -p sentra-ui --test demo_html`

Expected: PASS.

### Task 2: Demo Artifact Example

**Files:**
- Create: `crates/sentra-ui/examples/demo_dashboard.rs`

- [x] **Step 1: Add example source**

Create an example that builds a synthetic `DashboardState`, applies live telemetry, adds a pending action, renders HTML, creates `target/`, and writes `target/sentra-demo-dashboard.html`.

- [x] **Step 2: Run example**

Run: `cargo run -p sentra-ui --example demo_dashboard`

Expected: PASS and prints the generated HTML path.

### Task 3: Documentation And Verification

**Files:**
- Modify: `TASKS.md`
- Modify: `ARCHITECTURE.md`
- Modify: `PERFORMANCE_NOTES.md`
- Create: `PHASE_REPORTS/phase-19.md`
- Create: `TEST_RESULTS/phase-19.md`
- Modify: `docs/superpowers/plans/2026-06-28-phase-19-static-demo-dashboard.md`

- [x] **Step 1: Update docs**

Record that Phase 19 adds a static HTML demo dashboard renderer and example only.

- [x] **Step 2: Run full verification**

Run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
powershell -ExecutionPolicy Bypass -File tools\validate-architecture.ps1
powershell -ExecutionPolicy Bypass -File tools\run-quality-gates.ps1
```

Expected: every command exits 0.

- [x] **Step 3: Record results**

Update `TEST_RESULTS/phase-19.md` with targeted and final verification results.

- [x] **Step 4: Commit and push Omer**

Commit on `Omer`, then push only `Omer` to `origin/Omer`.
