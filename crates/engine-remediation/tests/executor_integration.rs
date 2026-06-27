use engine_remediation::{executor::RemediationExecutor, RemediationPlan, RemediationPlanStepKind};
use shared_models::{Alert, Finding, RiskLevel, RemediationAction, Timestamp, RemediationMode, process::{ProcessIdentity, ImagePath, CommandLine}};
use std::process::Command;
use std::time::Duration;

#[test]
#[cfg(target_os = "windows")]
fn test_executor_successfully_kills_process() {
    // 1. Spawn a dummy process (notepad.exe)
    let mut child = Command::new("notepad.exe")
        .spawn()
        .expect("Failed to spawn notepad.exe for testing");

    let pid = child.id();
    
    // Give it a moment to initialize
    std::thread::sleep(Duration::from_millis(500));

    // Verify it is still running
    assert!(child.try_wait().unwrap().is_none(), "Process died too early");

    // 2. Create a mock alert with the PID
    let mut finding = Finding::new(Timestamp::now(), RiskLevel::Critical, 100);
    finding.process = Some(ProcessIdentity {
        process_id: pid,
        parent_process_id: None,
        image_path: Some(ImagePath::new("notepad.exe")),
        command_line: Some(CommandLine::new("notepad.exe")),
        user_sid: None,
    });

    let alert = Alert::observe_only(finding, "test-kill");

    // 3. Create a kill plan
    let plan = RemediationPlan {
        alert_id: alert.alert_id.clone(),
        steps: vec![
            engine_remediation::RemediationPlanStep {
                kind: RemediationPlanStepKind::KillProcess,
                action: RemediationAction::KillProcess,
                description: "Kill Test".into(),
            }
        ],
        created_at: Timestamp::now(),
        plan_id: uuid::Uuid::new_v4(),
        mode: RemediationMode::ApprovalRequired,
    };

    // 4. Execute the kill plan
    let result = RemediationExecutor::execute_plan(&plan, &alert);
    assert!(result.is_ok(), "Executor failed to run plan: {:?}", result);

    // 5. Verify the process is dead
    // We might need to wait a tiny bit for the OS to reap it
    std::thread::sleep(Duration::from_millis(200));
    let status = child.wait().expect("Failed to wait on child");
    
    assert!(!status.success(), "Process exited normally instead of being killed");
    println!("SUCCESS: RemediationExecutor successfully killed the test process (PID: {})", pid);
}
