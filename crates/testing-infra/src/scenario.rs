#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    Synthetic,
    ControlledVm,
    Unsafe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioKind {
    TelemetryReplay,
    ProcessBehavior,
    PersistenceBehavior,
    NetworkBehavior,
    DetectionCorrelation,
    RemediationPlanning,
    MemoryMetadata,
    UiProjection,
    MalwareExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestScenario {
    pub name: String,
    pub kind: ScenarioKind,
    pub safety: SafetyLevel,
    pub phases: Vec<u8>,
    pub mitre_tags: Vec<String>,
}

impl TestScenario {
    pub fn new(
        name: impl Into<String>,
        kind: ScenarioKind,
        safety: SafetyLevel,
        phases: Vec<u8>,
        mitre_tags: Vec<&str>,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            safety,
            phases,
            mitre_tags: mitre_tags.into_iter().map(str::to_string).collect(),
        }
    }
}
