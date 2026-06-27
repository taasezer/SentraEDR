use crate::scenario::{SafetyLevel, ScenarioKind, TestScenario};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    UnsafeScenarioRejected(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioCatalog {
    pub scenarios: Vec<TestScenario>,
}

impl ScenarioCatalog {
    pub fn try_new(scenarios: Vec<TestScenario>) -> Result<Self, CatalogError> {
        for scenario in &scenarios {
            if scenario.safety == SafetyLevel::Unsafe {
                return Err(CatalogError::UnsafeScenarioRejected(scenario.name.clone()));
            }
        }

        Ok(Self { scenarios })
    }

    pub fn default_safe() -> Self {
        Self::try_new(vec![
            TestScenario::new(
                "synthetic process ETW lifecycle",
                ScenarioKind::TelemetryReplay,
                SafetyLevel::Synthetic,
                vec![2],
                vec!["T1059"],
            ),
            TestScenario::new(
                "powershell encoded command metadata",
                ScenarioKind::ProcessBehavior,
                SafetyLevel::Synthetic,
                vec![3],
                vec!["T1059.001"],
            ),
            TestScenario::new(
                "registry run key persistence metadata",
                ScenarioKind::PersistenceBehavior,
                SafetyLevel::Synthetic,
                vec![4],
                vec!["T1060"],
            ),
            TestScenario::new(
                "beacon-like network metadata",
                ScenarioKind::NetworkBehavior,
                SafetyLevel::Synthetic,
                vec![5],
                vec!["T1071"],
            ),
            TestScenario::new(
                "multi-signal detection correlation",
                ScenarioKind::DetectionCorrelation,
                SafetyLevel::Synthetic,
                vec![6],
                vec!["T1059.001", "T1071"],
            ),
            TestScenario::new(
                "approval-required remediation planning",
                ScenarioKind::RemediationPlanning,
                SafetyLevel::Synthetic,
                vec![7],
                vec!["T1562"],
            ),
            TestScenario::new(
                "remote thread memory metadata",
                ScenarioKind::MemoryMetadata,
                SafetyLevel::Synthetic,
                vec![8],
                vec!["T1055"],
            ),
            TestScenario::new(
                "dashboard alert summary projection",
                ScenarioKind::UiProjection,
                SafetyLevel::Synthetic,
                vec![9],
                vec!["T1059"],
            ),
        ])
        .expect("default safe catalog must not contain unsafe scenarios")
    }
}
