# ADR 0019: Component Manifest

## Status
Accepted

## Decision
Every crate exposing runtime functionality must expose a `ComponentManifest`. This manifest declares its dependencies, required capabilities, privileges, and startup/shutdown budgets. The Runtime reads these manifests to construct the dependency graph.
