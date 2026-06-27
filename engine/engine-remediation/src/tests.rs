#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::*;
    use crate::models::*;
    use crate::registry::ActionRegistry;
    use crate::providers::ActionProvider;
    use crate::errors::RemediationError;
    use uuid::Uuid;

    struct MockProvider;
    impl ActionProvider for MockProvider {
        fn provider_id(&self) -> &str { "MockProcessTerminator" }
        fn is_idempotent(&self) -> bool { true }
        fn execute(&self, _action: &RemediationAction) -> Result<(), RemediationError> { Ok(()) }
        fn verify(&self, _action: &RemediationAction) -> Result<(), RemediationError> { Ok(()) }
        fn generate_rollback(&self, _action: &RemediationAction) -> Result<Option<RollbackData>, RemediationError> { Ok(None) }
        fn dry_run(&self, _action: &RemediationAction) -> Result<(), RemediationError> { Ok(()) }
    }

    #[test]
    fn test_interactive_policy_blocks_execution() {
        let mut registry = ActionRegistry::new();
        registry.register(Box::new(MockProvider));
        
        let policy = SafetyPolicy {
            policy_id: "POL-01".to_string(),
            policy_version: "1.0".to_string(),
            operating_mode: OperatingMode::Interactive,
            requires_reversibility: false,
        };

        let pipeline = RemediationPipeline::new(&registry, &policy);
        
        let state1 = StateAlertReceived { alert_id: Uuid::new_v4() };
        let state2 = pipeline.plan(state1);
        
        // Interactive mode MUST return StatePendingApproval (Err) rather than SafetyValidated (Ok)
        let result = pipeline.check_approval(state2);
        assert!(result.is_err()); 
    }
}
