use crate::models::{RemediationAction, RollbackData};
use crate::errors::RemediationError;

pub trait ActionProvider {
    fn provider_id(&self) -> &str;
    fn is_idempotent(&self) -> bool;
    
    /// Executes the action and alters the system.
    fn execute(&self, action: &RemediationAction) -> Result<(), RemediationError>;
    
    /// Verifies that the system state matches the intended outcome post-execution.
    fn verify(&self, action: &RemediationAction) -> Result<(), RemediationError>;
    
    /// Generates the literal state required to reverse the action.
    fn generate_rollback(&self, action: &RemediationAction) -> Result<Option<RollbackData>, RemediationError>;
    
    /// Simulates the action without altering system state.
    fn dry_run(&self, action: &RemediationAction) -> Result<(), RemediationError>;
}
