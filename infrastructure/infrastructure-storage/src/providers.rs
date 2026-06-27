use crate::models::PersistedEvent;
use crate::errors::StorageError;

#[async_trait::async_trait]
pub trait StorageProvider: Send + Sync {
    /// Atomic batch insertion of events.
    async fn write_batch(&self, events: &[PersistedEvent]) -> Result<(), StorageError>;
    
    /// Queries events by timestamp range (Read Model).
    async fn query_range(&self, start_ms: u64, end_ms: u64) -> Result<Vec<PersistedEvent>, StorageError>;
}

/// A repository interface to prevent engines from leaking SQL dependencies.
#[async_trait::async_trait]
pub trait EventRepository: Send + Sync {
    async fn store(&self, event: PersistedEvent) -> Result<(), StorageError>;
}

pub struct InMemoryStorageProvider {
    // In a real implementation this would hold an Arc<Mutex<Vec<PersistedEvent>>>
}

impl InMemoryStorageProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl StorageProvider for InMemoryStorageProvider {
    async fn write_batch(&self, _events: &[PersistedEvent]) -> Result<(), StorageError> {
        // Mock success
        Ok(())
    }

    async fn query_range(&self, _start_ms: u64, _end_ms: u64) -> Result<Vec<PersistedEvent>, StorageError> {
        Ok(Vec::new())
    }
}
