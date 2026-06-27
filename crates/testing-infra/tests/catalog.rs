use testing_infra::{
    CoverageMatrix, SafetyLevel, ScenarioCatalog, ScenarioKind, TestScenario,
};

#[test]
fn default_catalog_contains_only_synthetic_safe_scenarios() {
    let catalog = ScenarioCatalog::default_safe();

    assert!(catalog.scenarios.len() >= 8);
    assert!(
        catalog
            .scenarios
            .iter()
            .all(|scenario| scenario.safety == SafetyLevel::Synthetic)
    );
}

#[test]
fn unsafe_scenarios_are_rejected() {
    let result = ScenarioCatalog::try_new(vec![TestScenario::new(
        "live malware sample",
        ScenarioKind::MalwareExecution,
        SafetyLevel::Unsafe,
        vec![10],
        vec!["T0000"],
    )]);

    assert!(result.is_err());
}

#[test]
fn coverage_matrix_reports_implemented_phase_coverage() {
    let catalog = ScenarioCatalog::default_safe();
    let matrix = CoverageMatrix::from_catalog(&catalog, 2..=9);

    assert!(matrix.missing_phases.is_empty());
    assert_eq!(matrix.coverage_for_phase(2).unwrap().scenario_count, 1);
    assert_eq!(matrix.coverage_for_phase(9).unwrap().scenario_count, 1);
}

#[test]
fn coverage_matrix_reports_missing_phase() {
    let catalog = ScenarioCatalog::try_new(vec![TestScenario::new(
        "synthetic process ETW lifecycle",
        ScenarioKind::TelemetryReplay,
        SafetyLevel::Synthetic,
        vec![2],
        vec!["T1059"],
    )])
    .unwrap();

    let matrix = CoverageMatrix::from_catalog(&catalog, 2..=3);

    assert_eq!(matrix.missing_phases, vec![3]);
}

#[test]
fn coverage_report_counts_scenarios_and_mitre_tags() {
    let catalog = ScenarioCatalog::default_safe();
    let matrix = CoverageMatrix::from_catalog(&catalog, 2..=9);
    let report = matrix.report();

    assert_eq!(report.total_phases, 8);
    assert_eq!(report.covered_phases, 8);
    assert!(report.total_scenarios >= 8);
    assert!(report.unique_mitre_tags >= 4);
}
