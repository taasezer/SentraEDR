use engine_memory::scanner::MemoryScanner;
use std::process;

#[test]
#[cfg(target_os = "windows")]
fn test_memory_scanner_detects_injected_memory() {
    use windows::Win32::System::Memory::{
        VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
    };

    let pid = process::id();

    // 1. Scan before allocation (baseline)
    let _initial_regions = MemoryScanner::scan_process(pid).expect("Failed to scan process");

    // 2. Allocate suspicious memory (Simulating Code Injection)
    let size = 4096;
    let addr = unsafe {
        VirtualAlloc(
            None,
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    
    assert!(!addr.is_null(), "Failed to allocate memory");

    // 3. Scan after allocation
    let new_regions = MemoryScanner::scan_process(pid).expect("Failed to scan process");

    // 4. Verify the scanner found our specific allocation
    let found = new_regions.iter().find(|r| r.base_address == addr as usize);
    
    assert!(found.is_some(), "Scanner failed to detect the injected memory region!");
    let suspicious_region = found.unwrap();
    
    assert!(suspicious_region.protection.contains("64"), "Expected PAGE_EXECUTE_READWRITE (64) but got {}", suspicious_region.protection);
    assert!(suspicious_region.state.contains("4096"), "Expected MEM_COMMIT (4096) but got {}", suspicious_region.state);
    assert!(suspicious_region.type_.contains("131072"), "Expected MEM_PRIVATE (131072) but got {}", suspicious_region.type_);

    println!("SUCCESS: Memory Scanner successfully caught the simulated injection at 0x{:X}", addr as usize);

    // Cleanup
    unsafe {
        let _ = VirtualFree(addr, 0, MEM_RELEASE);
    }
}
