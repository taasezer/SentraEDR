# Phase 17: Agent IPC Service Skeleton, In-Memory Only

## Overview
Phase 17 introduces the structural foundation for IPC (Inter-Process Communication) within the `sentra-agent`. While the transport layer (named pipes) is deferred, this phase establishes the lifecycle management, configuration, and in-memory orchestration of the IPC pipeline.

The goal is to transition from "IPC as a library" (`shared-ipc`) to "IPC as a service" within the agent.

## Architecture

### IPC Service Lifecycle
The `IpcService` will be managed by the `sentra-agent` runtime. Its primary responsibilities are:
1. **Initialization**: Loading IPC settings from `AgentConfig` and initializing the `IpcPipeline`.
2. **Data Flow**: Accepting raw byte chunks (simulated in this phase) and pushing them through the pipeline.
3. **Monitoring**: Periodically generating health and statistics summaries from the pipeline.
4. **Shutdown**: Gracefully closing the dispatcher queues.

### Configuration Changes
The `AgentConfig` will be extended to include an `IpcConfig` section:
- `enabled`: Boolean to toggle IPC functionality.
- `dispatcher_capacity`: The queue capacity to be passed to `IpcDispatcherConfig`.

### Integrated Flow (Synthetic Dry-Run)
In this phase, the agent will simulate a transport layer by feeding synthetic byte chunks into the `IpcService`. 
`Synthetic Bytes` $\rightarrow$ `IpcService` $\rightarrow$ `IpcPipeline` $\rightarrow$ `IpcDispatcher` $\rightarrow$ `Bounded Queues`.

## Interface

### `IpcService`
A new component in `sentra-agent` that wraps the `shared-ipc` pipeline.
- `new(config: IpcConfig) -> Self`
- `process_raw_bytes(&mut self, chunk: &[u8]) -> Result<(), IpcError>`
- `get_stats() -> IpcPipelineStats`

## Testing Strategy (TDD)
1. **Configuration Tests**: Verify that IPC settings are correctly loaded and validated.
2. **Service Integration Tests**: 
   - Feed raw bytes into `IpcService` and verify that messages reach the dispatcher queues.
   - Verify that `IpcPipelineStats` are correctly reflected in the `IpcService` health report.
3. **Dry-Run Validation**: Ensure the `sentra-agent` main loop can initialize and exercise the IPC service without crashing.

## Security and Constraints
- **In-Memory Only**: No sockets or pipes are opened.
- **Observe-Only**: The IPC service only routes data; it does not trigger any remediation actions.
- **Crate Boundaries**: `sentra-agent` depends on `shared-ipc`, but `shared-ipc` remains agnostic of the agent's internals.
