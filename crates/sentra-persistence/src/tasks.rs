use crate::make_detection;
use sentra_core::{DetectionResult, Result, SentraError, ThreatLevel};

pub struct ScheduledTaskInfo {
    pub name: String,
    pub status: String,
    pub next_run: String,
    pub last_run: String,
    pub author: String,
    pub task_to_run: String,
}

pub async fn enumerate_scheduled_tasks() -> Result<Vec<ScheduledTaskInfo>> {
    // Calling schtasks.exe /Query /FO CSV or using COM Task Scheduler APIs
    // Mock for prototype.
    Ok(Vec::new())
}

pub fn detect_suspicious_tasks(tasks: &[ScheduledTaskInfo]) -> Vec<DetectionResult> {
    let mut detections = Vec::new();

    for task in tasks {
        let run = task.task_to_run.to_lowercase();
        
        if run.contains("\\temp\\") || run.contains("-enc") || run.contains("-encodedcommand") {
            detections.push(make_detection(
                "Suspicious Scheduled Task",
                &format!("Task executes suspicious command: {}", task.name),
                ThreatLevel::High,
                0.8,
                &format!("Task: {}", task.task_to_run),
                Some("T1053.005"),
            ));
        }
    }

    detections
}
