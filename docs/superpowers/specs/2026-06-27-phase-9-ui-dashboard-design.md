# Phase 9 UI Dashboard Design

Date: 2026-06-27
Status: Approved for implementation by project roadmap continuation

## Goal

Phase 9 adds `sentra-ui`, a dashboard foundation crate that prepares UI-ready state from shared alert and remediation schemas. It does not render a web application yet, and it does not import agent or engine internals.

## Approach

The UI crate owns presentation state only. It accepts shared `Alert` values and UI-safe remediation review items, then builds a deterministic `DashboardState`.

This phase chooses a library-first UI foundation instead of a browser application because the repository does not yet have a live IPC server, frontend toolchain, or authenticated UI shell. The crate creates stable state contracts that a future web or desktop renderer can consume.

## Components

`sentra-ui::dashboard`

- Owns `DashboardState`.
- Tracks alert cards, risk summary counts, timeline entries, and pending action cards.

`sentra-ui::alert_card`

- Owns `AlertCard`.
- Converts shared `Alert` values into display-safe summaries.

`sentra-ui::timeline`

- Owns `TimelineEntry` and `TimelineKind`.
- Orders alert and remediation review events by timestamp.

`sentra-ui::action_queue`

- Owns `ActionReviewCard`.
- Represents approval-required remediation plans as reviewable UI state.

## Security Rules

- UI does not import any engine crate.
- UI does not import `sentra-agent`.
- UI does not create findings or alerts.
- UI does not approve or execute remediation.
- UI can display pending actions, but agent-side policy remains authoritative.

## Initial Behavior

- Alert cards show score, risk level, signal count, MITRE technique count, recommended action, and remediation eligibility.
- Risk summary counts informational, low, medium, high, and critical alerts.
- Timeline entries include alert observations and remediation review items.
- Pending action cards show waiting-for-approval plans without executing them.

## Testing

Tests cover:

- dashboard state from multiple alerts;
- risk summary aggregation;
- alert ordering by score;
- timeline ordering;
- pending remediation review cards;
- architecture validation that `sentra-ui` stays independent from engine and agent crates.

