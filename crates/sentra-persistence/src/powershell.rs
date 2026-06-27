use crate::make_detection;
use sentra_core::{DetectionResult, PowerShellEvent, ThreatLevel};

pub fn analyze_powershell_event(event: &PowerShellEvent) -> Vec<DetectionResult> {
    let mut detections = Vec::new();
    let text = event.script_block.to_lowercase();
    
    if text.contains("invoke-mimikatz") || text.contains("out-minidump") {
        detections.push(make_detection(
            "Malicious PowerShell Cmdlet",
            "Credential dumping cmdlet detected",
            ThreatLevel::Critical,
            0.95,
            &format!("Script snippet: {}", &event.script_block.chars().take(200).collect::<String>()),
            Some("T1003"),
        ));
    }
    
    if text.contains("[system.reflection.assembly]::load") {
        detections.push(make_detection(
            "PowerShell Reflection",
            "Dynamic assembly loading detected (.NET reflection)",
            ThreatLevel::High,
            0.7,
            "Assembly.Load used in script block",
            Some("T1055"),
        ));
    }
    
    if event.is_encoded {
        detections.push(make_detection(
            "Encoded PowerShell",
            "Encoded command execution detected",
            ThreatLevel::Medium,
            0.6,
            &format!("Command: {}", event.command_line),
            Some("T1027"),
        ));
    }

    detections
}
