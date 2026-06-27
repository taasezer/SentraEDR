//! # sentra-persistence
//!
//! Persistence mechanism detection for the **SentraEDR** Anti-RAT / EDR
//! platform.
//!
//! This crate monitors and detects the following Windows persistence
//! mechanisms:
//!
//! | Module          | Coverage |
//! |-----------------|----------|
//! | [`registry`]    | Registry Run/RunOnce keys, Winlogon, Shell Folders, IFEO |
//! | [`services`]    | Windows service enumeration and anomaly detection |
//! | [`tasks`]       | Scheduled task enumeration via `schtasks.exe` |
//! | [`startup`]     | User and common startup folder monitoring |
//! | [`wmi`]         | WMI permanent event subscription detection |
//! | [`powershell`]  | PowerShell script-block and command-line analysis |
//!
//! The top-level [`PersistenceMonitor`] orchestrates a full sweep across
//! all mechanisms and returns a unified list of [`DetectionResult`]s.

pub mod powershell;
pub mod registry;
pub mod services;
pub mod startup;
pub mod tasks;
pub mod wmi;

use chrono::Utc;
use sentra_core::error::{Result, SentraError};
use sentra_core::types::{DetectionResult, Evidence, ThreatLevel};
use tracing::{info, warn};
use uuid::Uuid;

/// Orchestrates a full persistence-mechanism sweep across the host.
///
/// Calling [`PersistenceMonitor::run_full_scan`] enumerates registry keys,
/// services, scheduled tasks, startup folders, and WMI subscriptions,
/// then applies heuristic rules to each category and returns a combined
/// list of detections.
///
/// # Example
///
/// ```no_run
/// # async fn demo() -> sentra_core::error::Result<()> {
/// let monitor = sentra_persistence::PersistenceMonitor::new();
/// let detections = monitor.run_full_scan().await?;
/// for det in &detections {
///     println!("[{}] {}", det.rule_name, det.description);
/// }
/// # Ok(())
/// # }
/// ```
pub struct PersistenceMonitor {
    /// Optional service baseline for tracking known-good services.
    service_baseline: Option<services::ServiceBaseline>,
}

impl PersistenceMonitor {
    /// Create a new [`PersistenceMonitor`] with no service baseline.
    pub fn new() -> Self {
        Self {
            service_baseline: None,
        }
    }

    /// Create a new [`PersistenceMonitor`] with an existing service baseline.
    pub fn with_baseline(baseline: services::ServiceBaseline) -> Self {
        Self {
            service_baseline: Some(baseline),
        }
    }

    /// Execute a full persistence scan across all mechanisms.
    ///
    /// Each sub-scanner runs independently; failures in one category are
    /// logged as warnings and do **not** prevent the other scanners from
    /// running.
    pub async fn run_full_scan(&self) -> Result<Vec<DetectionResult>> {
        let mut detections = Vec::new();

        // --- Registry ---
        match registry::scan_registry_persistence() {
            Ok(entries) => {
                info!(count = entries.len(), "Registry persistence entries enumerated");
                let mut reg_dets = registry::detect_suspicious_entries(&entries);
                detections.append(&mut reg_dets);
            }
            Err(e) => warn!(error = %e, "Registry persistence scan failed"),
        }

        // --- Services ---
        match services::enumerate_services() {
            Ok(svc_list) => {
                info!(count = svc_list.len(), "Services enumerated");
                let mut svc_dets =
                    services::detect_suspicious_services(&svc_list, self.service_baseline.as_ref());
                detections.append(&mut svc_dets);
            }
            Err(e) => warn!(error = %e, "Service enumeration failed"),
        }

        // --- Scheduled tasks ---
        match tasks::enumerate_scheduled_tasks().await {
            Ok(task_list) => {
                info!(count = task_list.len(), "Scheduled tasks enumerated");
                let mut task_dets = tasks::detect_suspicious_tasks(&task_list);
                detections.append(&mut task_dets);
            }
            Err(e) => warn!(error = %e, "Scheduled task enumeration failed"),
        }

        // --- Startup folders ---
        match startup::get_startup_folders() {
            Ok(folders) => {
                for folder in &folders {
                    match startup::scan_startup_folder(folder) {
                        Ok(entries) => {
                            info!(folder = %folder, count = entries.len(), "Startup entries scanned");
                            let mut su_dets = startup::detect_suspicious_startup(&entries);
                            detections.append(&mut su_dets);
                        }
                        Err(e) => warn!(folder = %folder, error = %e, "Startup folder scan failed"),
                    }
                }
            }
            Err(e) => warn!(error = %e, "Failed to resolve startup folders"),
        }

        // --- WMI ---
        match wmi::detect_wmi_persistence().await {
            Ok(mut wmi_dets) => {
                info!(count = wmi_dets.len(), "WMI persistence detections");
                detections.append(&mut wmi_dets);
            }
            Err(e) => warn!(error = %e, "WMI persistence scan failed"),
        }

        info!(total = detections.len(), "Persistence scan complete");
        Ok(detections)
    }
}

impl Default for PersistenceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to build a [`DetectionResult`] with common defaults.
pub(crate) fn make_detection(
    rule: &str,
    description: &str,
    threat: ThreatLevel,
    confidence: f64,
    evidence_detail: &str,
    mitre: Option<&str>,
) -> DetectionResult {
    DetectionResult {
        id: Uuid::new_v4(),
        rule_name: rule.to_owned(),
        threat_level: threat,
        confidence,
        description: description.to_owned(),
        evidence: vec![Evidence {
            source: "sentra-persistence".to_owned(),
            detail: evidence_detail.to_owned(),
            timestamp: Utc::now(),
        }],
        affected_process: None,
        timestamp: Utc::now(),
        mitre_technique: mitre.map(str::to_owned),
    }
}
