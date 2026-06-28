use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
use windows::Win32::Foundation::HANDLE;

/// Terminate a malicious process by its Process ID (PID)
pub fn kill_process(pid: u32) -> Result<(), String> {
    // Basic Safelist to prevent Blue Screen (BSOD) or system crash.
    // In a real EDR, this safelist checks code signatures, but we hardcode PIDs for safety.
    // 0 = System Idle, 4 = System, PID 4 cannot be killed anyway but it's good practice.
    if pid == 0 || pid == 4 {
        return Err("Cannot kill System processes".to_string());
    }

    unsafe {
        // Request PROCESS_TERMINATE access right
        let handle_result = OpenProcess(PROCESS_TERMINATE, false, pid);
        
        match handle_result {
            Ok(handle) => {
                // Instantly destroy the process
                let success = TerminateProcess(handle, 1);
                
                // Clean up the handle
                let _ = windows::Win32::Foundation::CloseHandle(handle);
                
                if success.is_ok() {
                    Ok(())
                } else {
                    Err(format!("TerminateProcess failed for PID {}", pid))
                }
            },
            Err(e) => {
                Err(format!("OpenProcess failed for PID {}: {}", pid, e))
            }
        }
    }
}
