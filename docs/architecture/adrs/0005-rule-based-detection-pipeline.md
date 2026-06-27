# ADR 0005: Rule-Based Detection Pipeline

## Status
Accepted

## Context
SentraEDR aims to be a deterministic, highly explainable security platform. Introducing Machine Learning, statistical anomaly detection, or AI-driven scoring at the foundation makes it impossible to guarantee absolute reproducibility for generated alerts.

## Decision
The Detection Engine will strictly employ a deterministic, rule-based pipeline for behavioral correlation. 
The pipeline consists of 6 isolated stages:
1. **Intake:** Ingesting snapshots from ETW, Process, Network, and Persistence engines.
2. **Correlation:** Organizing events into a searchable, bounded, in-memory state.
3. **Rule Evaluation:** Evaluating data-driven rules against the correlation state.
4. **Risk Scoring:** Assigning a quantitative threat value based on the matched rule logic.
5. **Confidence Evaluation:** Assigning a reliability metric to the alert (e.g. hash match = High Confidence, generic heuristic = Low Confidence).
6. **Alert Generation:** Formatting immutable `Alert` records.

## Alternatives Considered
- **Statistical Anomaly Detection:** While powerful for zero-days, statistical deviations generate excessive false positives and cannot be easily explained to a human analyst in an incident response scenario.
- **Monolithic Evaluation:** Combining scoring, correlation, and alerting into a single function. This violates the testability mandate.

## Trade-offs
- *Pros:* 100% reproducible alerts. Simple to test. Simple to write external rule packs.
- *Cons:* Fails to detect completely novel behaviors that have no predefined signature or heuristic rule.

## Consequences
All rules must provide their own explicit correlation windows (e.g., this rule needs 5 seconds of context, this one needs 24 hours). The Correlation Engine will aggressively cull state to respect these boundaries.
