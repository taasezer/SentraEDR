use crate::state::ProcessSnapshot;
use shared_models::TelemetryEventId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSignal {
    pub name: String,
    pub description: String,
    pub severity: SignalSeverity,
    pub process: ProcessSnapshot,
    pub parent: Option<ProcessSnapshot>,
    pub supporting_event_id: TelemetryEventId,
}

pub fn signals_for_start(
    process: &ProcessSnapshot,
    parent: Option<&ProcessSnapshot>,
    supporting_event_id: TelemetryEventId,
) -> Vec<ProcessSignal> {
    let mut signals = Vec::new();

    if let Some(parent) = parent
        && is_suspicious_parent_child(parent, process)
    {
        signals.push(ProcessSignal {
            name: "suspicious_parent_child".to_string(),
            description: "Process lineage matched a suspicious parent-child pair".to_string(),
            severity: SignalSeverity::High,
            process: process.clone(),
            parent: Some(parent.clone()),
            supporting_event_id: supporting_event_id.clone(),
        });
    }

    if has_powershell_encoded_command(process) {
        signals.push(ProcessSignal {
            name: "powershell_encoded_command".to_string(),
            description: "PowerShell command line contains an encoded command flag".to_string(),
            severity: SignalSeverity::Medium,
            process: process.clone(),
            parent: parent.cloned(),
            supporting_event_id: supporting_event_id.clone(),
        });
    }

    if runs_from_user_writable_path(process) {
        signals.push(ProcessSignal {
            name: "user_writable_execution_path".to_string(),
            description: "Process image path appears to be under a user-writable location"
                .to_string(),
            severity: SignalSeverity::Medium,
            process: process.clone(),
            parent: parent.cloned(),
            supporting_event_id,
        });
    }

    signals
}

fn is_suspicious_parent_child(parent: &ProcessSnapshot, child: &ProcessSnapshot) -> bool {
    let Some(parent_name) = executable_name(parent) else {
        return false;
    };
    let Some(child_name) = executable_name(child) else {
        return false;
    };

    matches!(
        (parent_name.as_str(), child_name.as_str()),
        ("winword.exe", "powershell.exe")
            | ("excel.exe", "powershell.exe")
            | ("powerpnt.exe", "powershell.exe")
            | ("chrome.exe", "powershell.exe")
            | ("msedge.exe", "powershell.exe")
            | ("firefox.exe", "powershell.exe")
            | ("winrar.exe", "powershell.exe")
            | ("7z.exe", "powershell.exe")
    )
}

fn has_powershell_encoded_command(process: &ProcessSnapshot) -> bool {
    let Some(name) = executable_name(process) else {
        return false;
    };
    if name != "powershell.exe" && name != "pwsh.exe" {
        return false;
    }

    let Some(command_line) = process.process.command_line.as_ref() else {
        return false;
    };
    let command_line = command_line.as_str().to_ascii_lowercase();

    command_line.contains("-enc")
        || command_line.contains("/enc")
        || command_line.contains("-encodedcommand")
        || command_line.contains("/encodedcommand")
}

fn runs_from_user_writable_path(process: &ProcessSnapshot) -> bool {
    let Some(image_path) = process.process.image_path.as_ref() else {
        return false;
    };
    let image_path = image_path.as_str().replace('/', r"\").to_ascii_lowercase();

    image_path.contains(r"\appdata\local\temp\")
        || image_path.contains(r"\appdata\roaming\")
        || image_path.contains(r"\downloads\")
        || image_path.contains(r"\temp\")
        || image_path.contains(r"\users\")
}

fn executable_name(process: &ProcessSnapshot) -> Option<String> {
    let image_path = process.process.image_path.as_ref()?.as_str();
    image_path
        .rsplit(['\\', '/'])
        .next()
        .map(|name| name.to_ascii_lowercase())
}
