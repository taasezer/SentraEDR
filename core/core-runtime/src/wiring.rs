pub struct WiringReport {
    pub instantiated_components: Vec<String>,
    pub dependency_graph: String,
    pub registered_capabilities: Vec<String>,
}

pub struct ProgressiveWiring {
    // Progressively registers:
    // Telemetry -> Process -> Network -> Persistence -> Detection -> Remediation -> Storage -> Communication
}
