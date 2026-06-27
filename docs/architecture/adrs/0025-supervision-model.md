# ADR 0025: Supervision Model

## Status
Accepted

## Decision
All asynchronous tasks must be spawned via a `Supervisor` rather than raw `tokio::spawn`. The Supervisor manages the `RestartPolicy` declared in the ComponentManifest (Never, OnFailure, Always) and applies exponential backoff for crash-looping services.
