use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Schema Mismatch: {0}")]
    SchemaMismatch(String),
    #[error("Transaction Failed: {0}")]
    TransactionFailed(String),
    #[error("Validation Error: {0}")]
    ValidationError(String),
}
