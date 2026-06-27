use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemediationError {
    #[error("Validation Failure: {0}")]
    ValidationFailure(String),
    #[error("Execution Failure: {0}")]
    ExecutionFailure(String),
    #[error("Verification Failure: {0}")]
    VerificationFailure(String),
    #[error("Rollback Failure: {0}")]
    RollbackFailure(String),
    #[error("Policy Failure: {0}")]
    PolicyFailure(String),
}
