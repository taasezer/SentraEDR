use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetectionEngineError {
    #[error("Rule Evaluation Error: {0}")]
    RuleError(String),
    #[error("State Exceeded Bounds: {0}")]
    StateBoundsExceeded(String),
}
