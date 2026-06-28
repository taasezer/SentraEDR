use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // 1. Build sentra-service (Release mode)
    println!("cargo:warning=Building sentra-service sidecar...");
    let status = Command::new("cargo")
        .args(&["build", "--release", "--bin", "sentra-service"])
        .current_dir("../../../") // Root workspace dir
        .status()
        .expect("Failed to execute cargo build for sentra-service");

    if !status.success() {
        panic!("Failed to build sentra-service");
    }

    // 2. Determine target triple to copy the sidecar with the correct suffix
    let target = env::var("TARGET").unwrap_or_else(|_| "x86_64-pc-windows-msvc".to_string());
    let source_path = Path::new("../../../target/release/sentra-service.exe");
    
    // Tauri looks for bin/sentra-service-<TARGET>.exe
    let bin_dir = Path::new("bin");
    if !bin_dir.exists() {
        fs::create_dir_all(bin_dir).expect("Failed to create bin dir");
    }
    
    let target_path = bin_dir.join(format!("sentra-service-{}.exe", target));
    fs::copy(source_path, target_path).expect("Failed to copy sidecar binary");

    // 3. Set UAC Administrator privilege in the Windows manifest
    let mut windows = tauri_build::WindowsAttributes::new();
    windows = windows.app_manifest(r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*" />
    </dependentAssembly>
  </dependency>
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/PM</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>
"#);

    let attrs = tauri_build::Attributes::new().windows_attributes(windows);
    tauri_build::try_build(attrs).expect("failed to run tauri-build");
}
