use uuid::Uuid;

pub struct AgentHealthView {
    pub is_healthy: bool,
    pub uptime_seconds: u64,
    pub loaded_rules: usize,
    pub memory_usage_mb: u64,
}

pub struct AlertView {
    pub alert_id: Uuid,
    pub rule_name: String,
    pub confidence: u32,
    pub risk_score: u32,
    pub mitre_tactic: String,
}

pub struct DashboardState {
    pub agent_health: AgentHealthView,
    pub recent_alerts: Vec<AlertView>,
}
