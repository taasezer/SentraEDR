use crate::{RemediationPlan, RemediationPlanStepKind};
use shared_models::Alert;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ExecutorError {
    ProcessNotFound,
    AccessDenied,
    NativeError(u32),
    IoError(String),
    MissingImagePath,
}

impl fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessNotFound => write!(f, "Process not found"),
            Self::AccessDenied => write!(f, "Access denied"),
            Self::NativeError(code) => write!(f, "Native Windows error code: {}", code),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::MissingImagePath => write!(f, "Missing image path for quarantine"),
        }
    }
}

pub struct RemediationExecutor;

impl RemediationExecutor {
    pub fn execute_plan(plan: &RemediationPlan, alert: &Alert) -> Result<(), ExecutorError> {
        let process = match &alert.finding.process {
            Some(p) => p,
            None => return Err(ExecutorError::ProcessNotFound),
        };
        let pid = process.process_id;
        
        for step in &plan.steps {
            match step.kind {
                RemediationPlanStepKind::KillProcess => {
                    Self::kill_process(pid)?;
                }
                RemediationPlanStepKind::SuspendProcess => {
                    // MVP: map suspend to kill for now
                    Self::kill_process(pid)?;
                }
                RemediationPlanStepKind::QuarantineFile => {
                    if let Some(path) = &process.image_path {
                        Self::quarantine_file(path.as_str())?;
                    } else {
                        return Err(ExecutorError::MissingImagePath);
                    }
                }
                RemediationPlanStepKind::DeleteFile => {
                    if let Some(path) = &process.image_path {
                        Self::delete_file(path.as_str())?;
                    } else {
                        return Err(ExecutorError::MissingImagePath);
                    }
                }
                _ => {
                    // Ignore other unimplemented actions for MVP
                }
            }
        }
        Ok(())
    }

    fn kill_process(pid: u32) -> Result<(), ExecutorError> {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Foundation::CloseHandle;
            use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
            
            unsafe {
                let handle_result = OpenProcess(PROCESS_TERMINATE, false, pid);
                let handle = match handle_result {
                    Ok(h) => h,
                    Err(e) => return Err(ExecutorError::NativeError(e.code().0 as u32)),
                };
                
                let term_result = TerminateProcess(handle, 1);
                let _ = CloseHandle(handle);
                
                if let Err(e) = term_result {
                    return Err(ExecutorError::NativeError(e.code().0 as u32));
                }
            }
        }
        
        Ok(())
    }

    fn quarantine_file(path: &str) -> Result<(), ExecutorError> {
        let original_path = std::path::Path::new(path);
        if !original_path.exists() {
            return Err(ExecutorError::IoError("File does not exist".into()));
        }
        
        let quarantine_path = format!("{}.quarantined", path);
        if let Err(e) = std::fs::rename(original_path, &quarantine_path) {
            return Err(ExecutorError::IoError(e.to_string()));
        }
        
        Ok(())
    }

    fn delete_file(path: &str) -> Result<(), ExecutorError> {
        let original_path = std::path::Path::new(path);
        if !original_path.exists() {
            // If it doesn't exist anymore, nothing to delete
            return Ok(());
        }
        
        if let Err(e) = std::fs::remove_file(original_path) {
            return Err(ExecutorError::IoError(e.to_string()));
        }
        
        Ok(())
    }
}
