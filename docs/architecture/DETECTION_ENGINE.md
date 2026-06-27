# SentraEDR Detection Engine

## Overview
The detection engine is the central brain of SentraEDR. It correlates events from `engine-etw`, `engine-process`, `engine-network`, and `engine-persistence` to identify threat patterns.

## Detection Strategy
Instead of relying on static signatures, SentraEDR utilizes heuristic scoring and behavioral mapping against the MITRE ATT&CK framework.

### Scoring System
Events are assigned a risk score based on their nature:
- Unsigned binary spawning from `AppData`: +30
- Outbound network connection to an unknown IP shortly after creation: +20
- Creation of a Run registry key by the same process: +40
- **Total Score:** 90 (Triggers high-confidence alert)

### Attack Chain Correlation
The engine tracks state over time for a given process tree. 
- Process tracking is maintained in a low-overhead, eviction-based cache.
- Network and persistence events are linked back to the originating Process ID (PID).

### False Positive Suppression
- Known-good signatures and publishers are whitelisted.
- Common behaviors (e.g., legitimate updaters) are suppressed through contextual heuristics.
- Requires multi-signal agreement (e.g., network + persistence) to reach critical thresholds automatically.
