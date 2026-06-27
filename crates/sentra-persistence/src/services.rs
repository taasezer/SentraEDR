use crate::make_detection;
use sentra_core::{DetectionResult, Result, SentraError, ServiceInfo, ServiceStartType, ServiceStatus, ThreatLevel};
use std::collections::HashSet;

pub struct ServiceBaseline {
    pub known_services: HashSet<String>,
}

pub fn enumerate_services() -> Result<Vec<ServiceInfo>> {
    // Requires elevation and complex SCManager API usage.
    // For this prototype, we return an empty list or mock.
    Ok(Vec::new())
}

pub fn detect_suspicious_services(services: &[ServiceInfo], baseline: Option<&ServiceBaseline>) -> Vec<DetectionResult> {
    let mut detections = Vec::new();

    for svc in services {
        let bin_path = svc.binary_path.to_lowercase();
        
        if bin_path.contains("\\temp\\") || bin_path.contains("\\users\\") && bin_path.contains("\\appdata\\") {
            detections.push(make_detection(
                "Suspicious Service Binary Path",
                &format!("Service {} points to user/temp directory", svc.name),
                ThreatLevel::High,
                0.8,
                &format!("Binary: {}", svc.binary_path),
                Some("T1543.003"),
            ));
        }

        if let Some(b) = baseline {
            if !b.known_services.contains(&svc.name) {
                detections.push(make_detection(
                    "New Unknown Service",
                    &format!("Newly created service detected: {}", svc.name),
                    ThreatLevel::Medium,
                    0.5,
                    &format!("Binary: {}", svc.binary_path),
                    Some("T1543.003"),
                ));
            }
        }
    }

    detections
}
