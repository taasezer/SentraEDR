use sentra_core::{Result, SentraError};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Memory::{
    VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_PRIVATE, PAGE_EXECUTE_READWRITE,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub struct MemoryRegion {
    pub base_address: u64,
    pub size: u64,
    pub state: u32,
    pub protection: u32,
    pub region_type: u32,
    pub is_executable: bool,
    pub is_writable: bool,
}

pub fn scan_memory_regions(pid: u32) -> Result<Vec<MemoryRegion>> {
    let mut regions = Vec::new();

    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
            .map_err(|_| SentraError::WindowsApi("OpenProcess failed for memory scan".into()))?;

        let mut address: usize = 0;
        let mut mem_info = MEMORY_BASIC_INFORMATION::default();

        while VirtualQueryEx(
            process_handle,
            Some(address as *const _),
            &mut mem_info,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        ) != 0
        {
            regions.push(MemoryRegion {
                base_address: mem_info.BaseAddress as u64,
                size: mem_info.RegionSize as u64,
                state: mem_info.State.0,
                protection: mem_info.Protect.0,
                region_type: mem_info.Type.0,
                is_executable: (mem_info.Protect.0 & PAGE_EXECUTE_READWRITE.0) != 0,
                is_writable: (mem_info.Protect.0 & PAGE_EXECUTE_READWRITE.0) != 0,
            });

            address += mem_info.RegionSize;
        }

        let _ = CloseHandle(process_handle);
    }

    Ok(regions)
}

pub fn detect_rwx_regions(pid: u32) -> Result<Vec<MemoryRegion>> {
    let regions = scan_memory_regions(pid)?;
    Ok(regions
        .into_iter()
        .filter(|r| r.protection == PAGE_EXECUTE_READWRITE.0 && r.state == MEM_COMMIT.0)
        .collect())
}

pub fn detect_unbacked_executable(pid: u32) -> Result<Vec<MemoryRegion>> {
    let regions = scan_memory_regions(pid)?;
    Ok(regions
        .into_iter()
        .filter(|r| r.is_executable && r.region_type == MEM_PRIVATE.0 && r.state == MEM_COMMIT.0)
        .collect())
}
