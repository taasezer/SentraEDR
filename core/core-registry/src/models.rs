#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityStatus {
    Registered,
    Initialized,
    Ready,
    Disabled,
    ShuttingDown,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unavailable,
}

#[derive(Debug, Clone)]
pub struct CapabilityId(pub String); // Strongly typed identifier

#[derive(Debug, Clone)]
pub struct CapabilityMetadata {
    pub id: CapabilityId,
    pub version: String,
    pub category: String, // e.g., "Telemetry", "Storage", "Detection"
    pub dependencies: Vec<CapabilityId>,
    pub status: CapabilityStatus,
    pub health: HealthStatus,
}

pub trait Capability {
    fn metadata(&self) -> CapabilityMetadata;
    fn set_status(&mut self, status: CapabilityStatus);
    fn initialize(&mut self) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
}
