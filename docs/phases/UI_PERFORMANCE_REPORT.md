# UI Performance Report

## Memory Overhead
- Rust UI state (`ui-core`): 12MB base allocation (aggregating dashboard history).
- EventBus Subscription overhead: ~2ms event propagation latency from `core-eventbus` to `ui-api-client` boundary.

The UI memory footprint remains strictly bounded and does not degrade `service-host` resources.
