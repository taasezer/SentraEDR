# UI Rendering Architecture

## Real-Time Visualization Strategy
The UI must never attempt to render raw telemetry streams directly (e.g., rendering every individual file write).

### Virtualization
- Alert streams and process trees must utilize virtualization (rendering only the visible viewport items).

### Aggregation
- The backend `EventBus` emits aggregated `DetectionSummaryUpdated` events (e.g., 100 FileWrites per second summarized into a 1Hz sparkline event).
- UI charting libraries will render these aggregated 1Hz sparklines rather than calculating raw telemetry on the frontend thread.
