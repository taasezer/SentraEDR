# Phase 16: IPC Stream Assembler + Frame Intake Pipeline Composition

## Overview
This phase implements the composition of the IPC processing pipeline. It bridges the gap between raw byte streams (received from a transport layer, though the transport itself is not yet implemented) and the dispatching of high-level IPC messages.

The pipeline implements a linear flow:
`Raw Bytes` -> `IpcStreamAssembler` (Frames) -> `IpcFrameIntake` (Envelopes) -> `IpcDispatcher` (Queues).

## Architecture

### Pipeline Flow
1. **Input**: A chunk of raw bytes (`&[u8]`).
2. **Assembly**: The `IpcStreamAssembler` buffers bytes and identifies complete frames based on the length-prefixed codec.
3. **Intake**: Each complete frame is passed to `IpcFrameIntake`.
4. **Decoding**: `IpcFrameIntake` decodes the frame into an `IpcEnvelope`.
5. **Dispatch**: The `IpcEnvelope` is routed by `IpcDispatcher` to the appropriate bounded channel.

### Data Structures

#### `IpcPipeline`
The main orchestrator of the IPC flow.
- `assembler: IpcStreamAssembler`
- `intake: IpcFrameIntake`
- `stats: IpcPipelineStats`

#### `IpcPipelineStats`
Aggregated metrics for the pipeline:
- `chunks_received`: Total number of byte chunks processed.
- `frames_completed`: Total frames successfully assembled by the assembler.
- `frames_accepted`: Total frames successfully decoded and dispatched.
- `stream_rejected`: Total stream errors (buffer overflow or frame size violation).
- `intake_decode_failed`: Total frames that failed decoding.
- `intake_dispatch_failed`: Total envelopes that failed dispatch.

## Interface

### `IpcPipeline::process_bytes(&mut self, chunk: &[u8]) -> Result<(), IpcError>`
- Increments `chunks_received`.
- Calls `assembler.push_bytes(chunk)`.
- If `push_bytes` returns an error (e.g., `StreamBufferTooLarge`), increments `stream_rejected` and returns the error.
- For each frame produced by the assembler:
    - Increments `frames_completed`.
    - Calls `intake.accept_frame(&frame)`.
    - If `accept_frame` returns an error:
        - If it's a decode error, increments `intake_decode_failed`.
        - If it's a dispatch error, increments `intake_dispatch_failed`.
        - (The error is logged/returned based on policy, but the pipeline continues processing other frames in the chunk).
    - If successful, increments `frames_accepted`.

## Error Handling
The pipeline is designed to be resilient. A failure in decoding one frame should not necessarily stop the processing of subsequent frames in the same chunk, although a critical stream error (like buffer overflow) must stop the process.

## Testing Strategy (TDD)
1. **Unit Tests**: Create tests in `crates/shared-ipc/tests/pipeline.rs`.
2. **Scenario 1: Happy Path**: Send a complete frame split across multiple chunks. Verify it reaches the dispatcher.
3. **Scenario 2: Fragmented Frames**: Send multiple frames split across chunks. Verify all are dispatched.
4. **Scenario 3: Malformed Frames**: Send a frame that fails decoding. Verify `intake_decode_failed` increments and the pipeline recovers.
5. **Scenario 4: Buffer Overflow**: Send a chunk that exceeds `MAX_BUFFERED_BYTES`. Verify `stream_rejected` increments.
6. **Scenario 5: Dispatch Failure**: Mock a full queue. Verify `intake_dispatch_failed` increments.
