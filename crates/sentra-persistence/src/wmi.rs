use crate::make_detection;
use sentra_core::{DetectionResult, Result, SentraError, ThreatLevel};

pub async fn detect_wmi_persistence() -> Result<Vec<DetectionResult>> {
    // Requires WMI queries (e.g. via `wmi` crate or PowerShell interop)
    // Query: SELECT * FROM __EventFilter
    // Query: SELECT * FROM __EventConsumer
    // Query: SELECT * FROM __FilterToConsumerBinding
    Ok(Vec::new())
}
