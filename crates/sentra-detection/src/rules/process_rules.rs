use chrono::Utc;
use sentra_core::{DetectionResult, Detector, Evidence, TelemetryEvent, ThreatLevel};
use uuid::Uuid;

pub struct SuspiciousProcessCreator;

impl Detector for SuspiciousProcessCreator {
    fn name(&self) -> &str {
        "SuspiciousProcessCreator"
    }

    fn analyze(&self, event: &TelemetryEvent) -> Option<DetectionResult> {
        if let TelemetryEvent::ProcessCreate(proc) = event {
            let p_name = proc.name.to_lowercase();
            // In a real implementation we would have the parent process name here from the process tree
            
            if proc.cmdline.to_lowercase().contains("vssadmin delete shadows") {
                return Some(DetectionResult {
                    id: Uuid::new_v4(),
                    rule_name: "Ransomware Shadow Copy Deletion".to_string(),
                    threat_level: ThreatLevel::Critical,
                    confidence: 0.95,
                    description: "A process attempted to delete volume shadow copies".to_string(),
                    evidence: vec![Evidence {
                        source: "process_monitor".to_string(),
                        detail: format!("Command line: {}", proc.cmdline),
                        timestamp: Utc::now(),
                    }],
                    affected_process: Some(proc.clone()),
                    timestamp: Utc::now(),
                    mitre_technique: Some("T1490".to_string()),
                });
            }
        }
        None
    }

    fn threat_categories(&self) -> Vec<String> {
        vec!["Execution".to_string(), "Impact".to_string()]
    }
}
