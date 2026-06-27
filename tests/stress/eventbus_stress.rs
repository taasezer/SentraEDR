#[tokio::test]
async fn test_eventbus_flooding() {
    // 1. Initialize EventBus
    // 2. Spawn 100 threads firing 1,000,000 events
    // 3. Validate bounded channels do not cause OOM and gracefully drop lowest-priority telemetry
}
