use std::fmt;

#[derive(Debug, Clone)]
pub enum ScannerError {
    ProcessNotFound,
    AccessDenied,
    NativeError(u32),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessNotFound => write!(f, "Process not found"),
            Self::AccessDenied => write!(f, "Access denied"),
            Self::NativeError(code) => write!(f, "Native Windows error code: {}", code),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SuspiciousRegion {
    pub base_address: usize,
    pub size: usize,
    pub protection: String,
    pub state: String,
    pub type_: String,
    pub reason: String,
}

pub struct MemoryScanner;

impl MemoryScanner {
    pub fn scan_process(pid: u32) -> Result<Vec<SuspiciousRegion>, ScannerError> {
        let mut suspicious = Vec::new();

        #[cfg(target_os = "windows")]
        {
            use windows::Win32::Foundation::CloseHandle;
            use windows::Win32::System::Memory::{
                VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_PRIVATE,
                PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
            };
            use windows::Win32::System::Threading::{
                OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
            };

            unsafe {
                let handle_result = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);
                let handle = match handle_result {
                    Ok(h) => h,
                    Err(e) => return Err(ScannerError::NativeError(e.code().0 as u32)),
                };

                let mut current_address = 0usize;
                let mut mem_info = MEMORY_BASIC_INFORMATION::default();

                while VirtualQueryEx(
                    handle,
                    Some(current_address as *const _),
                    &mut mem_info,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                ) != 0
                {
                    // Look for Unbacked Executable Memory (Private + Execute)
                    if mem_info.State == MEM_COMMIT
                        && mem_info.Type == MEM_PRIVATE
                        && (mem_info.Protect == PAGE_EXECUTE_READWRITE
                            || mem_info.Protect == PAGE_EXECUTE_READ)
                    {
                        suspicious.push(SuspiciousRegion {
                            base_address: mem_info.BaseAddress as usize,
                            size: mem_info.RegionSize,
                            protection: format!("{:?}", mem_info.Protect),
                            state: format!("{:?}", mem_info.State),
                            type_: format!("{:?}", mem_info.Type),
                            reason: "Unbacked Executable Memory (Potential Code Injection)".to_string(),
                        });
                    }

                    // Move to the next page
                    current_address = mem_info.BaseAddress as usize + mem_info.RegionSize;
                }

                let _ = CloseHandle(handle);
            }
        }

        Ok(suspicious)
    }
}
