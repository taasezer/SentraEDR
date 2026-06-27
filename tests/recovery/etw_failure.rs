#[tokio::test]
async fn test_etw_session_disconnect() {
    // 1. Setup RuntimeBuilder with mocked ETW
    // 2. Inject failure disconnecting the kernel session
    // 3. Verify Supervisor detects state change to `Degraded`
    // 4. Verify exponential backoff attempts reconnection
}
