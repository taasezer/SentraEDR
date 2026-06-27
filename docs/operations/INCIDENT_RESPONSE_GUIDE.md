# Incident Response Guide

## Diagnostic Extraction
When investigating false positives or agent performance degradation, administrators can request a Diagnostic Bundle via the `CommandBus` (`GenerateDiagnosticsCommand`). 

## Log Collection
The bundle automatically includes:
- Current Rule Registry state.
- Provider ETW health metrics.
- Active Supervisor crash loop logs.
- SQLite WAL checkpoint state.
- Last 1,000 internal diagnostic events from the Windows Event Log.

## Detection Tuning
Detections are tuned dynamically by submitting an `UpdateConfigurationCommand` that whitelists specific Image Paths or Hashes globally. Rules do not require a restart to adopt new tuning parameters.
