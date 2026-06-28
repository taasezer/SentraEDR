use engine_file::FileAnalyzer;
use shared_models::Timestamp;

#[test]
fn test_file_analyzer_detects_ryuk() {
    let analyzer = FileAnalyzer::new();
    
    // Normal file should pass
    let result_normal = analyzer.analyze_file_io("C:\\temp\\document.txt", 1234, Timestamp::now());
    assert!(result_normal.is_none());

    // Ransomware extension should trigger
    let result_ryuk = analyzer.analyze_file_io("C:\\Users\\user\\Desktop\\file.ryuk", 1234, Timestamp::now());
    assert!(result_ryuk.is_some());
    let signal = result_ryuk.unwrap();
    
    assert_eq!(signal.extension, "ryuk");
    assert_eq!(signal.pid, 1234);

    println!("SUCCESS: FileAnalyzer successfully caught the .ryuk extension!");
}
