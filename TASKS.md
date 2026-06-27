# SentraEDR Task Tracker

## PHASE 0: Research and Planning
- [x] Define full architecture
- [x] Define IPC strategy
- [x] Define telemetry pipeline
- [x] Define memory budget
- [x] Define detection strategy
- [x] Create architecture diagrams (Architecture.md)
- [x] Create threat model
- [x] Create security model
- [x] Create telemetry model
- [x] Create roadmap

## PHASE 1: Workspace and architecture initialization
- [x] Initialize monorepo skeleton
- [x] Initialize Rust workspace & Cargo.toml metadata
- [x] Initialize modular crates
- [x] Validate workspace (cargo check)
- [x] Verify crate boundaries and dependency graph
- [ ] *DEFERRED:* Initialize Tauri UI (Moved to Phase 9)
- [ ] *DEFERRED:* Initialize IPC framework (Moved to Phase 2/3 dependencies)
- [ ] *DEFERRED:* Initialize SQLite layer (Moved to Phase 4)

## PHASE 2: ETW telemetry engine
- [ ] Realtime ETW subscriptions
- [ ] Process creation events
- [ ] Image load events
- [ ] Registry events
- [ ] PowerShell events
- [ ] Thread creation events

## PHASE 3: Process monitoring engine
- [ ] Process enumeration
- [ ] Parent-child analysis
- [ ] Unsigned binary detection
- [ ] Suspicious execution path detection
- [ ] AppData execution detection
- [ ] Hidden PowerShell detection

## PHASE 4: Persistence engine
- [ ] Run key detection
- [ ] Startup folder analysis
- [ ] Scheduled task analysis
- [ ] Service analysis
- [ ] WMI persistence detection

## PHASE 5: Network engine
- [ ] Outbound connection analysis
- [ ] Suspicious IP tracking
- [ ] DNS anomaly detection
- [ ] Beacon interval analysis
- [ ] Connection graphing

## PHASE 6: Heuristic detection engine
- [ ] Scoring engine
- [ ] Risk classification
- [ ] Behavior correlation
- [ ] Attack chain correlation
- [ ] False positive suppression

## PHASE 7: Quarantine and remediation engine
- [ ] Isolate process
- [ ] Suspend process
- [ ] Backup registry
- [ ] Remove persistence
- [ ] Quarantine binaries
- [ ] Rollback support

## PHASE 8: Memory inspection engine
- [ ] Injected DLL detection
- [ ] RWX memory scanning
- [ ] Suspicious thread analysis
- [ ] Shellcode heuristics

## PHASE 9: UI dashboard
- [ ] Realtime monitoring dashboard
- [ ] Detection timeline
- [ ] Process graph
- [ ] Telemetry graph
- [ ] Alert system
- [ ] Remediation controls

## PHASE 10: Testing infrastructure
- [ ] VM testing
- [ ] Atomic Red Team integration
- [ ] Safe malware simulation
- [ ] Regression testing
- [ ] Performance benchmarking
