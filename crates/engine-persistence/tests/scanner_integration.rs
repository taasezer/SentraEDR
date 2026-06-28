use engine_persistence::scanner::PersistenceScanner;

#[test]
#[cfg(target_os = "windows")]
fn test_persistence_scanner_detects_suspicious_run_key() {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key_path = r#"Software\Microsoft\Windows\CurrentVersion\Run"#;
    
    // 1. Create a suspicious Run key pointing to AppData
    let run_key = hkcu.open_subkey_with_flags(run_key_path, KEY_WRITE).expect("Failed to open Run key");
    let test_value_name = "SentraTestRAT";
    let test_value_data = r#"C:\Users\FakeUser\AppData\Roaming\update.exe"#;
    
    run_key.set_value(test_value_name, &test_value_data).expect("Failed to set Run key");

    // 2. Run the scanner
    let suspicious_entries = PersistenceScanner::scan_run_keys().expect("Failed to scan run keys");

    // 3. Verify it found our test key
    let found = suspicious_entries.iter().find(|e| e.value_name == test_value_name);
    assert!(found.is_some(), "Scanner failed to detect the suspicious Run key");
    
    let entry = found.unwrap();
    assert_eq!(entry.target_path, test_value_data);
    assert!(entry.reason.contains("AppData"));

    println!("SUCCESS: Persistence Scanner successfully caught the suspicious Registry Run key!");

    // Cleanup
    let _ = run_key.delete_value(test_value_name);
}
