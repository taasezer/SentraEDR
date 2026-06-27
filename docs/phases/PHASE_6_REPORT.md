# Phase 6: Detection Engine Report

## Completed Work
- **ADRs Created:** 
  - `0005-rule-based-detection-pipeline.md`: Enshrined the deterministic, explainable, 6-stage detection pipeline without ML abstractions.
- **Detection Engine Implementation (`engine-detection`):**
  - **Models (`models.rs`):** Explicitly modeled the immutable `Alert`, uncoupling `severity` (risk) from `confidence`. Built the `Evidence` struct to reference `EventId`s rather than duplicating massive payloads.
  - **Rules (`rules.rs`):** Introduced the generic `Rule` trait, pushing the `max_correlation_window_ms` TTL requirement directly into the rule logic rather than relying on a global fixed window.
  - **Pipeline (`pipeline.rs`):** Designed the `DetectionPipeline` and `CorrelationState`. The state dynamically enforces memory bounding by continuously purging events older than the longest required rule TTL.
  - **Metrics (`metrics.rs`):** Added observability for rule evaluation latency and correlation memory bytes.
- **Testing:**
  - Designed `tests.rs` with synthetic events injected across time windows to mathematically prove the `CorrelationState` successfully purges expired events based on Rule TTLs.
- **Documentation:**
  - `DETECTION_ARCHITECTURE.md`: Documented the entire rule lifecycle, explainability mandates, and correlation limits.
  - `HEALTH_REPORT.md`: Produced the mandatory Architecture Health Assessment.

## Architectural Enforcement
- The detection engine is 100% decoupled from OS queries. It does not import `windows` crates.
- State correlation cannot grow without bounds.

## Next Steps
- We have now stabilized ETW, Process, Persistence, Network, and Detection. 
- The user has mandated that after these five engines stabilize, we can evaluate storage or remediation.
