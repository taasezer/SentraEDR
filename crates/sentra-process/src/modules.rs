use sentra_core::{DllLoadInfo, Result, SentraError};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::{CloseHandle, MAX_PATH};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
};

pub fn enumerate_modules(pid: u32, process_name: &str) -> Result<Vec<DllLoadInfo>> {
    let mut modules = Vec::new();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)
            .map_err(|e| SentraError::WindowsApi(format!("CreateToolhelp32Snapshot failed: {}", e)))?;

        let mut entry = MODULEENTRY32W {
            dwSize: std::mem::size_of::<MODULEENTRY32W>() as u32,
            ..Default::default()
        };

        if Module32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                let dll_name = get_string_from_u16(&entry.szModule);
                let dll_path = get_string_from_u16(&entry.szExePath);

                modules.push(DllLoadInfo {
                    pid,
                    process_name: process_name.to_string(),
                    dll_path,
                    dll_name,
                    base_address: entry.modBaseAddr as u64,
                    size: entry.modBaseSize as u64,
                    is_signed: false, // In a real implementation we would check Authenticode here
                });

                if Module32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);
    }

    Ok(modules)
}

pub fn detect_suspicious_modules(modules: &[DllLoadInfo]) -> Vec<DllLoadInfo> {
    modules.iter()
        .filter(|m| {
            let path = m.dll_path.to_lowercase();
            path.contains("\\temp\\") || path.contains("\\appdata\\local\\temp\\")
        })
        .cloned()
        .collect()
}

pub fn detect_unsigned_modules(modules: &[DllLoadInfo]) -> Vec<&DllLoadInfo> {
    modules.iter().filter(|m| !m.is_signed).collect()
}

fn get_string_from_u16(slice: &[u16]) -> String {
    let len = slice.iter().take_while(|&&c| c != 0).count();
    OsString::from_wide(&slice[..len]).to_string_lossy().into_owned()
}
