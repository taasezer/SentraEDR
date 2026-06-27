use crate::catalog::ScenarioCatalog;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PhaseCoverage {
    pub phase: u8,
    pub scenario_count: usize,
    pub mitre_tags: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoverageReport {
    pub total_phases: usize,
    pub covered_phases: usize,
    pub total_scenarios: usize,
    pub unique_mitre_tags: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoverageMatrix {
    pub phases: Vec<PhaseCoverage>,
    pub missing_phases: Vec<u8>,
}

impl CoverageMatrix {
    pub fn from_catalog(catalog: &ScenarioCatalog, phases: RangeInclusive<u8>) -> Self {
        let mut coverage = Vec::new();
        let mut missing_phases = Vec::new();

        for phase in phases {
            let scenarios_for_phase: Vec<_> = catalog
                .scenarios
                .iter()
                .filter(|scenario| scenario.phases.contains(&phase))
                .collect();
            if scenarios_for_phase.is_empty() {
                missing_phases.push(phase);
            }

            let mut mitre_tags = BTreeSet::new();
            for scenario in &scenarios_for_phase {
                for tag in &scenario.mitre_tags {
                    mitre_tags.insert(tag.clone());
                }
            }

            coverage.push(PhaseCoverage {
                phase,
                scenario_count: scenarios_for_phase.len(),
                mitre_tags: mitre_tags.into_iter().collect(),
            });
        }

        Self {
            phases: coverage,
            missing_phases,
        }
    }

    pub fn coverage_for_phase(&self, phase: u8) -> Option<&PhaseCoverage> {
        self.phases.iter().find(|coverage| coverage.phase == phase)
    }

    pub fn report(&self) -> CoverageReport {
        let mut unique_tags = BTreeSet::new();
        for phase in &self.phases {
            for tag in &phase.mitre_tags {
                unique_tags.insert(tag.clone());
            }
        }

        CoverageReport {
            total_phases: self.phases.len(),
            covered_phases: self
                .phases
                .iter()
                .filter(|phase| phase.scenario_count > 0)
                .count(),
            total_scenarios: self
                .phases
                .iter()
                .map(|phase| phase.scenario_count)
                .sum(),
            unique_mitre_tags: unique_tags.len(),
        }
    }
}
