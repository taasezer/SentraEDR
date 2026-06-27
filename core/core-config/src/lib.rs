pub struct FeatureFlags {
    pub enable_advanced_heuristics: bool,
    pub enable_network_capture: bool,
    pub enable_auto_remediation: bool,
}

pub struct LayeredConfiguration {
    pub active_flags: FeatureFlags,
    // Loaded via defaults -> config files -> env vars -> cli overrides
}
