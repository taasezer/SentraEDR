# Phase 6 Heuristic Detection Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `engine-detection`, an observe-only heuristic correlation engine that turns preliminary signals into scored findings and alerts.

**Architecture:** `engine-detection` depends only on `shared-models`. `sentra-agent` runs a synthetic dry run using detection input signals and logs finding/alert counts.

**Tech Stack:** Rust 1.85+ edition 2024, existing workspace crates, `shared-models`.

---

## Tasks

- [ ] Add `engine-detection` with failing correlation tests.
- [ ] Implement signal model, scoring, finding generation, and observe-only alert generation.
- [ ] Add agent synthetic detection dry run and test.
- [ ] Update docs, validator, reports, and final verification.
