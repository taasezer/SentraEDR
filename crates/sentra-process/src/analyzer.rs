use sentra_core::{ProcessInfo, ThreatLevel};

pub enum CmdlineIndicatorType {
    EncodedPowerShell,
    DownloadCradle,
    Base64Payload,
    SuspiciousFlag,
    LolBin,
    ObfuscatedCommand,
}

pub struct CmdlineIndicator {
    pub indicator_type: CmdlineIndicatorType,
    pub detail: String,
    pub severity: ThreatLevel,
}

pub fn analyze_cmdline(proc: &ProcessInfo) -> Vec<CmdlineIndicator> {
    let mut indicators = Vec::new();
    let cmd = proc.cmdline.to_lowercase();

    if proc.name.to_lowercase() == "powershell.exe" || proc.name.to_lowercase() == "pwsh.exe" {
        if cmd.contains("-enc") || cmd.contains("-encodedcommand") || cmd.contains("-e ") {
            indicators.push(CmdlineIndicator {
                indicator_type: CmdlineIndicatorType::EncodedPowerShell,
                detail: "Encoded PowerShell command found".to_string(),
                severity: ThreatLevel::High,
            });
        }
        if cmd.contains("invoke-webrequest") || cmd.contains("iwr ") || cmd.contains("net.webclient") || cmd.contains("downloadstring") {
            indicators.push(CmdlineIndicator {
                indicator_type: CmdlineIndicatorType::DownloadCradle,
                detail: "PowerShell download cradle found".to_string(),
                severity: ThreatLevel::High,
            });
        }
    }

    if proc.name.to_lowercase() == "certutil.exe" && cmd.contains("-urlcache") && cmd.contains("-split") {
        indicators.push(CmdlineIndicator {
            indicator_type: CmdlineIndicatorType::DownloadCradle,
            detail: "certutil download pattern found".to_string(),
            severity: ThreatLevel::High,
        });
    }

    if proc.name.to_lowercase() == "rundll32.exe" {
        if cmd.contains("javascript:") || cmd.contains("vbscript:") {
            indicators.push(CmdlineIndicator {
                indicator_type: CmdlineIndicatorType::LolBin,
                detail: "rundll32 executing script".to_string(),
                severity: ThreatLevel::High,
            });
        }
    }

    if proc.name.to_lowercase() == "regsvr32.exe" && cmd.contains("/i:") && cmd.contains("/s") {
        indicators.push(CmdlineIndicator {
            indicator_type: CmdlineIndicatorType::LolBin,
            detail: "regsvr32 network payload execution pattern".to_string(),
            severity: ThreatLevel::High,
        });
    }

    indicators
}
