pub struct ComponentManifest {
    pub component_id: String,
    pub name: String,
    pub version: String,
    pub category: String,
    pub dependencies: Vec<String>,
    pub capabilities: Vec<String>,
    pub health_checks: Vec<String>,
    pub required_configuration: Vec<String>,
    pub supported_platforms: Vec<String>,
    
    // Phase 10 additions
    pub startup_priority: u32,
    pub shutdown_priority: u32,
    pub estimated_startup_time_ms: u64,
    pub required_privileges: Vec<String>,
    pub feature_flags: Vec<String>,
    pub restart_policy: RestartPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

#[derive(Clone)]
pub struct RuntimeContext {
    // Passed to engines instead of global singletons
    // pub event_bus: Arc<dyn EventBus>, // Generic bus
    // pub registry: Arc<CapabilityRegistry>,
}
