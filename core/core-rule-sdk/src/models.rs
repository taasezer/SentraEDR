use uuid::Uuid;

pub struct MitreAttackMapping {
    pub tactic: String,
    pub technique: String,
    pub sub_technique: Option<String>,
}

pub struct PerformanceBudget {
    pub max_execution_time_us: u64,
    pub max_allocations: u64,
    pub max_memory_bytes: u64,
}

pub struct RuleManifest {
    pub rule_id: Uuid,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub mitre_mapping: Vec<MitreAttackMapping>,
    pub required_capabilities: Vec<String>,
    pub performance_budget: PerformanceBudget,
}
