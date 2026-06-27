use std::ops::Drop;
use windows::Win32::Foundation::{HANDLE, CloseHandle};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION};
use crate::errors::ProcessEngineError;

/// Strict RAII wrapper for Windows HANDLEs.
/// Ensures `CloseHandle` is always called when the object goes out of scope.
pub struct SafeHandle(HANDLE);

impl Drop for SafeHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

/// Abstract OS interaction for process queries.
pub struct ProcessQuerySource;

impl ProcessQuerySource {
    /// Attempts to open a process with query rights.
    /// Explicitly handles ACCESS_DENIED as a structured error rather than panicking.
    pub fn open_process(pid: u32) -> Result<SafeHandle, ProcessEngineError> {
        unsafe {
            // Simulated implementation: OpenProcess usually takes PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid
            let handle_result = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid);
            match handle_result {
                Ok(h) => Ok(SafeHandle(h)),
                Err(e) => {
                    // Win32 ERROR_ACCESS_DENIED is 0x80070005. 
                    if e.code().0 == 0x80070005_u32 as i32 {
                        Err(ProcessEngineError::AccessDenied(pid))
                    } else {
                        Err(ProcessEngineError::OsApiFailure(e.to_string()))
                    }
                }
            }
        }
    }
}
