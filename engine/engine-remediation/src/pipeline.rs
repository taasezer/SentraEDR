use crate::models::*;
use crate::registry::ActionRegistry;
use crate::errors::RemediationError;
use uuid::Uuid;

pub struct RemediationPipeline<'a> {
    registry: &'a ActionRegistry,
    policy: &'a SafetyPolicy,
}

impl<'a> RemediationPipeline<'a> {
    pub fn new(registry: &'a ActionRegistry, policy: &'a SafetyPolicy) -> Self {
        Self { registry, policy }
    }

    // Explicit state transitions
    
    pub fn plan(&self, state: StateAlertReceived) -> StateActionPlanned {
        StateActionPlanned {
            action: RemediationAction {
                action_id: Uuid::new_v4(),
                alert_id: state.alert_id,
                provider_id: "MockProcessTerminator".to_string(),
                payload: "1234".to_string(),
            }
        }
    }

    pub fn check_approval(&self, state: StateActionPlanned) -> Result<StateSafetyValidated, StatePendingApproval> {
        match self.policy.operating_mode {
            OperatingMode::Interactive | OperatingMode::AuditOnly => Err(StatePendingApproval { action: state.action }),
            OperatingMode::Automatic => Ok(StateSafetyValidated { action: state.action, rollback: None }),
        }
    }

    pub fn validate_safety(&self, mut state: StateSafetyValidated) -> Result<StateExecuting, RemediationError> {
        let provider = self.registry.get(&state.action.provider_id)
            .ok_or_else(|| RemediationError::ValidationFailure("Provider not found".to_string()))?;
        
        if self.policy.requires_reversibility {
            let rollback = provider.generate_rollback(&state.action)?;
            state.rollback = rollback;
        }

        Ok(StateExecuting { action: state.action, rollback: state.rollback })
    }

    pub fn execute(&self, state: StateExecuting) -> Result<StateVerification, RemediationError> {
        let provider = self.registry.get(&state.action.provider_id).unwrap();
        
        if self.policy.operating_mode == OperatingMode::AuditOnly {
            provider.dry_run(&state.action)?;
        } else {
            provider.execute(&state.action)?;
        }

        Ok(StateVerification { action: state.action, rollback: state.rollback })
    }

    pub fn verify(&self, state: StateVerification) -> Result<StateCompleted, RemediationError> {
        let provider = self.registry.get(&state.action.provider_id).unwrap();
        provider.verify(&state.action)?;

        Ok(StateCompleted {
            audit: AuditRecord {
                alert_id: state.action.alert_id,
                action_id: state.action.action_id,
                timestamp_ms: 1000,
                policy_id: self.policy.policy_id.clone(),
                result_status: "Verified".to_string(),
                rollback_reference: state.rollback.map(|r| r.action_id),
                integrity_hash: "hash_placeholder".to_string(),
            }
        })
    }
}
