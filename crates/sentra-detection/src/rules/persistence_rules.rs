use chrono::Utc;
use sentra_core::{DetectionResult, Detector, Evidence, TelemetryEvent, ThreatLevel};
use uuid::Uuid;

pub struct RegistryPersistenceMonitor;

impl Detector for RegistryPersistenceMonitor {
    fn name(&self) -> &str {
        "RegistryPersistenceMonitor"
    }

    fn analyze(&self, event: &TelemetryEvent) -> Option<DetectionResult> {
        if let TelemetryEvent::RegistryModify(reg) = event {
            let key = reg.key_path.to_lowercase();
            if key.contains("currentversion\\run") || key.contains("currentversion\\runonce") {
                if let Some(data) = &reg.data {
                    if data.to_lowercase().contains("cmd.exe") || data.to_lowercase().contains("powershell.exe") {
                        return Some(DetectionResult {
                            id: Uuid::new_v4(),
                            rule_name: "Suspicious Registry Run Key".to_string(),
                            threat_level: ThreatLevel::High,
                            confidence: 0.9,
                            description: "Run key created pointing to a command interpreter".to_string(),
                            evidence: vec![Evidence {
                                source: "registry_monitor".to_string(),
                                detail: format!("Key: {} Value: {} Data: {}", reg.key_path, reg.value_name, data),
                                timestamp: Utc::now(),
                            }],
                            affected_process: None,
                            timestamp: Utc::now(),
                            mitre_technique: Some("T1547.001".to_string()),
                        });
                    }
                }
            }
        }
        None
    }

    fn threat_categories(&self) -> Vec<String> {
        vec!["Persistence".to_string()]
    }
}
