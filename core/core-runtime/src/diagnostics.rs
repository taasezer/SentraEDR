pub struct BootstrapReport {
    pub startup_duration_ms: u64,
    pub initialized_services: Vec<String>,
    pub dependency_graph_valid: bool,
    pub enabled_feature_flags: Vec<String>,
}

pub struct DiagnosticReport {
    pub failing_component: String,
    pub lifecycle_stage: String,
    pub root_cause: String,
    pub recovery_suggestion: String,
}
