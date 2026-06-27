use crate::{RemediationPlan, RemediationPlanStepKind};
use shared_models::Alert;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ExecutorError {
    ProcessNotFound,
    AccessDenied,
    NativeError(u32),
}

impl fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessNotFound => write!(f, "Process not found"),
            Self::AccessDenied => write!(f, "Access denied"),
            Self::NativeError(code) => write!(f, "Native Windows error code: {}", code),
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
                    // MVP: map suspend to kill for now, or just implement suspend if needed
                    Self::kill_process(pid)?;
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
}
