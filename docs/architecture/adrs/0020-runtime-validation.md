# ADR 0020: Runtime Validation

## Status
Accepted

## Decision
Before moving the Runtime from `Validating` to `Initializing`, the `RuntimeBuilder` must validate the dependency graph, duplicate identifiers, and configuration schemas. Any failure immediately panics the boot process with structured diagnostics.
