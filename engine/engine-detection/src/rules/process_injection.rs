use crate::pipeline::CorrelationState;
use crate::models::Alert;
use crate::rules::Rule;
use shared_models::events::EventType;
use uuid::Uuid;

pub struct ProcessInjectionRule;

impl Rule for ProcessInjectionRule {
    fn name(&self) -> &'static str {
        "Process Injection (Remote Thread Creation)"
    }

    fn description(&self) -> &'static str {
        "Detects a process allocating and executing a thread inside a different, remote process. Common in sophisticated RATs (Hollowing/Injection)."
    }

    fn max_correlation_window_ms(&self) -> u64 {
        0 // Single event rule
    }

    fn evaluate(&self, state: &CorrelationState) -> Option<Alert> {
        let latest = state.events.back()?;

        if let EventType::RemoteThreadCreate { target_process_id } = &latest.event_type {
            // attacker_pid is the process that spawned the thread. In native_parser, we assigned it to `process_id`.
            let attacker_pid = latest.process_id;

            // In a real EDR, we check signatures here to ignore System/Antivirus creating remote threads.
            // For now, we alert immediately!
            return Some(Alert {
                alert_id: Uuid::new_v4(),
                rule_id: "EDR-INJECTION-001".to_string(),
                severity: 90, // High severity (90/100) triggers Auto-Kill
                confidence: 90,
                timestamp_ms: chrono::Utc::now().timestamp_millis() as u64,
                related_process_id: Some(attacker_pid),
                evidence: crate::models::Evidence {
                    related_event_ids: vec![latest.event_id.clone()],
                    reasoning_path: format!("Process {} injected a remote thread into Target Process {}. Possible RAT/Backdoor.", attacker_pid, target_process_id),
                }
            });
        }
        None
    }
}
