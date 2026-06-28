use crate::pipeline::CorrelationState;
use crate::models::Alert;
use crate::rules::Rule;
use shared_models::events::EventType;
use uuid::Uuid;

pub struct RegistryPersistenceRule;

impl Rule for RegistryPersistenceRule {
    fn name(&self) -> &'static str {
        "Registry Persistence (Auto-Start Backdoor)"
    }

    fn description(&self) -> &'static str {
        "Detects modifications to Windows Run / RunOnce keys, commonly used by malware to survive reboots."
    }

    fn max_correlation_window_ms(&self) -> u64 {
        0 // Single event rule
    }

    fn evaluate(&self, state: &CorrelationState) -> Option<Alert> {
        let latest = state.events.back()?;

        if let EventType::RegistryActivity { key_path, value_name, action } = &latest.event_type {
            if action == "SetValue" {
                let path_lower = key_path.to_lowercase();
                
                // Check if the modified registry key is a known persistence location
                if path_lower.contains("currentversion\\run") || path_lower.contains("currentversion\\runonce") {
                    
                    return Some(Alert {
                        alert_id: Uuid::new_v4(),
                        rule_id: "EDR-PERSISTENCE-001".to_string(),
                        severity: 85, // High triggers Auto-Kill
                        confidence: 90,
                        timestamp_ms: chrono::Utc::now().timestamp_millis() as u64,
                        related_process_id: Some(latest.process_id),
                        evidence: crate::models::Evidence {
                            related_event_ids: vec![latest.event_id.clone()],
                            reasoning_path: format!("Process {} attempted to establish persistence by writing to Startup Registry: {} -> {}", latest.process_id, key_path, value_name),
                        }
                    });
                }
            }
        }
        None
    }
}
