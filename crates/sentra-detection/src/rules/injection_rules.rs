use chrono::Utc;
use sentra_core::{DetectionResult, Detector, Evidence, TelemetryEvent, ThreatLevel};
use uuid::Uuid;

pub struct UnsignedDllLoader;

impl Detector for UnsignedDllLoader {
    fn name(&self) -> &str {
        "UnsignedDllLoader"
    }

    fn analyze(&self, event: &TelemetryEvent) -> Option<DetectionResult> {
        if let TelemetryEvent::DllLoad(dll) = event {
            if !dll.is_signed {
                let name = dll.dll_name.to_lowercase();
                if name.ends_with(".tmp") || dll.dll_path.to_lowercase().contains("\\temp\\") {
                    return Some(DetectionResult {
                        id: Uuid::new_v4(),
                        rule_name: "Suspicious Unsigned DLL Load".to_string(),
                        threat_level: ThreatLevel::High,
                        confidence: 0.8,
                        description: "An unsigned DLL was loaded from a temporary directory".to_string(),
                        evidence: vec![Evidence {
                            source: "module_monitor".to_string(),
                            detail: format!("DLL path: {}", dll.dll_path),
                            timestamp: Utc::now(),
                        }],
                        affected_process: None, // We'd ideally link the ProcessInfo here
                        timestamp: Utc::now(),
                        mitre_technique: Some("T1055".to_string()),
                    });
                }
            }
        }
        None
    }

    fn threat_categories(&self) -> Vec<String> {
        vec!["Defense Evasion".to_string()]
    }
}
