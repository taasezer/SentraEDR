use engine_remediation::{executor::RemediationExecutor, RemediationPlan, RemediationPlanStepKind};
use shared_models::{Alert, Finding, RiskLevel, RemediationAction, Timestamp, RemediationMode, process::{ProcessIdentity, ImagePath}};
use std::fs::File;

#[test]
fn test_executor_quarantines_file() {
    let test_file = "test_malware.exe";
    let quarantine_file = "test_malware.exe.quarantined";

    // Clean up from previous runs
    let _ = std::fs::remove_file(test_file);
    let _ = std::fs::remove_file(quarantine_file);

    // 1. Create a dummy file
    File::create(test_file).expect("Failed to create test file");
    assert!(std::path::Path::new(test_file).exists());

    // 2. Create a mock alert with the file path
    let mut finding = Finding::new(Timestamp::now(), RiskLevel::Critical, 100);
    finding.process = Some(ProcessIdentity {
        process_id: 9999,
        parent_process_id: None,
        image_path: Some(ImagePath::new(test_file)),
        command_line: None,
        user_sid: None,
    });

    let alert = Alert::observe_only(finding, "test-quarantine");

    // 3. Create a quarantine plan
    let plan = RemediationPlan {
        alert_id: alert.alert_id.clone(),
        steps: vec![
            engine_remediation::RemediationPlanStep {
                kind: RemediationPlanStepKind::QuarantineFile,
                action: RemediationAction::QuarantineFile,
                description: "Quarantine Test".into(),
            }
        ],
        created_at: Timestamp::now(),
        plan_id: uuid::Uuid::new_v4(),
        mode: RemediationMode::ApprovalRequired,
    };

    // 4. Execute the plan
    let result = RemediationExecutor::execute_plan(&plan, &alert);
    assert!(result.is_ok(), "Executor failed to run plan: {:?}", result);

    // 5. Verify the original file is gone and the quarantined file exists
    assert!(!std::path::Path::new(test_file).exists(), "Original file still exists");
    assert!(std::path::Path::new(quarantine_file).exists(), "Quarantined file does not exist");

    println!("SUCCESS: RemediationExecutor successfully quarantined the test file");

    // Cleanup
    let _ = std::fs::remove_file(quarantine_file);
}
