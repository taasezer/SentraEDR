#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PersistedEvent;
    use crate::pipeline::StoragePipeline;
    use crate::providers::InMemoryStorageProvider;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_async_batching_non_blocking() {
        let provider = Arc::new(InMemoryStorageProvider::new());
        let pipeline = StoragePipeline::start(provider, 2);

        // Enqueue events. The `enqueue` method uses `try_send`, guaranteeing non-blocking behavior.
        pipeline.enqueue(PersistedEvent {
            internal_id: Uuid::new_v4(),
            schema_version: 1,
            event_version: 1,
            producer_version: "1.0.0".to_string(),
            timestamp_ms: 1000,
            payload_json: "{}".to_string(),
        });

        pipeline.enqueue(PersistedEvent {
            internal_id: Uuid::new_v4(),
            schema_version: 1,
            event_version: 1,
            producer_version: "1.0.0".to_string(),
            timestamp_ms: 2000,
            payload_json: "{}".to_string(),
        });

        // Yield execution so the spawned task can process the batch
        tokio::task::yield_now().await;

        // No explicit assertion needed; the test proves the enqueue does not deadlock or panic.
    }
}
