use crate::models::Alert;
use crate::pipeline::CorrelationState;
use shared_models::events::EventType;
use std::collections::HashMap;

pub trait Rule: Send + Sync {
    fn rule_id(&self) -> &str;
    fn max_correlation_window_ms(&self) -> u64;
    fn evaluate(&self, state: &CorrelationState) -> Option<Alert>;
}

pub struct LsassDumpRule;

impl Rule for LsassDumpRule {
    fn rule_id(&self) -> &str {
        "EDR-CRED-001"
    }

    fn max_correlation_window_ms(&self) -> u64 {
        0
    }

    fn evaluate(&self, state: &CorrelationState) -> Option<Alert> {
        if let Some(event) = state.events.back() {
            if let EventType::ProcessCreate { command_line, image_path } = &event.event_type {
                let cmd = command_line.to_lowercase();
                
                // Common LSASS dump patterns
                let is_comsvcs = cmd.contains("comsvcs.dll") && cmd.contains("minidump");
                let is_procdump = image_path.to_lowercase().contains("procdump") && cmd.contains("lsass");

                if is_comsvcs || is_procdump {
                    return Some(Alert {
                        alert_id: uuid::Uuid::new_v4(),
                        rule_id: self.rule_id().to_string(),
                        timestamp_ms: event.timestamp_ms,
                        severity: 100, // Critical
                        confidence: 95, // Very High Confidence
                        related_process_id: Some(event.process_id),
                        evidence: crate::models::Evidence {
                            related_event_ids: vec![event.event_id],
                            reasoning_path: format!("Detected LSASS Memory Dump Attempt: {}", command_line),
                        }
                    });
                }
            }
        }
        None
    }
}

pub struct ReverseShellRule;

impl Rule for ReverseShellRule {
    fn rule_id(&self) -> &str {
        "EDR-NET-002"
    }

    fn max_correlation_window_ms(&self) -> u64 {
        10000 // Look back 10 seconds for the process creation
    }

    fn evaluate(&self, state: &CorrelationState) -> Option<Alert> {
        if let Some(recent_event) = state.events.back() {
            if let EventType::NetworkConnection { destination_ip, destination_port, protocol } = &recent_event.event_type {
                
                // If a network connection happened, check if the PID belongs to powershell/cmd
                let pid = recent_event.process_id;
                
                // Search backward for the ProcessCreate event of this PID
                for historical_event in state.events.iter().rev().skip(1) {
                    if historical_event.process_id == pid {
                        if let EventType::ProcessCreate { image_path, .. } = &historical_event.event_type {
                            let image = image_path.to_lowercase();
                            if image.contains("powershell.exe") || image.contains("pwsh.exe") || image.contains("cmd.exe") {
                                
                                return Some(Alert {
                                    alert_id: uuid::Uuid::new_v4(),
                                    rule_id: self.rule_id().to_string(),
                                    timestamp_ms: recent_event.timestamp_ms,
                                    severity: 85, // High
                                    confidence: 80, 
                                    related_process_id: Some(pid),
                                    evidence: crate::models::Evidence {
                                        related_event_ids: vec![historical_event.event_id, recent_event.event_id],
                                        reasoning_path: format!("Interactive shell {} initiated an outbound {} connection to {}:{}", image, protocol, destination_ip, destination_port),
                                    }
                                });
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

pub struct RansomwareBehaviorRule;

impl Rule for RansomwareBehaviorRule {
    fn rule_id(&self) -> &str {
        "EDR-FILE-003"
    }

    fn max_correlation_window_ms(&self) -> u64 {
        5000 // Track file writes over 5 seconds
    }

    fn evaluate(&self, state: &CorrelationState) -> Option<Alert> {
        if let Some(recent_event) = state.events.back() {
            if let EventType::FileActivity { file_path, action } = &recent_event.event_type {
                
                let lower_path = file_path.to_lowercase();
                if action == "Write" || action == "Rename" {
                    // 1. Direct Extension Check (Hard Match)
                    if lower_path.ends_with(".encrypted") || lower_path.ends_with(".lock") || lower_path.ends_with(".wncry") {
                        return Some(Alert {
                            alert_id: uuid::Uuid::new_v4(),
                            rule_id: self.rule_id().to_string(),
                            timestamp_ms: recent_event.timestamp_ms,
                            severity: 100, // Critical
                            confidence: 100, 
                            related_process_id: Some(recent_event.process_id),
                            evidence: crate::models::Evidence {
                                related_event_ids: vec![recent_event.event_id],
                                reasoning_path: format!("Ransomware extension detected during file write/rename: {}", file_path),
                            }
                        });
                    }

                    // We removed the heuristic (50 files in 5 seconds) because standard Windows processes
                    // (like npm, compilers, browsers) easily write hundreds of files per second, 
                    // causing massive false positives. We now only rely on the hard extension match above.
                }
            }
        }
        None
    }
}
