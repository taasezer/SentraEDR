use chrono::{DateTime, Utc};
use sentra_core::{DetectionResult, Evidence, NetworkConnection, ThreatLevel};
use std::collections::HashMap;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TimestampedConnection {
    pub connection: NetworkConnection,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub connection_count: u64,
}

pub struct NetworkAnomalyDetector {
    pub beaconing_min_connections: u64,
    pub beaconing_max_jitter: f64,
}

impl NetworkAnomalyDetector {
    pub fn new(min_conns: u64, max_jitter: f64) -> Self {
        Self {
            beaconing_min_connections: min_conns,
            beaconing_max_jitter: max_jitter,
        }
    }

    pub fn detect_suspicious_ports(&self, conn: &NetworkConnection) -> Option<DetectionResult> {
        if let Some(remote_addr) = conn.remote_addr {
            let port = remote_addr.port();
            
            // Common RAT/Backdoor ports
            let is_suspicious = match port {
                4444 | 5555 | 1337 | 31337 | 8888 | 9999 | 6666 | 7777 | 1234 | 12345 | 54321 => true,
                4443 | 8443 => true, // Metasploit defaults
                27374 | 6711 | 6712 | 6713 => true, // Sub7 / old trojans
                _ => false,
            };

            if is_suspicious {
                return Some(DetectionResult {
                    id: Uuid::new_v4(),
                    rule_name: "Suspicious Remote Port".to_string(),
                    threat_level: ThreatLevel::High,
                    confidence: 0.8,
                    description: format!("Process {} connected to suspicious port {}", conn.pid, port),
                    evidence: vec![Evidence {
                        source: "network_monitor".to_string(),
                        detail: format!("Remote address: {}", remote_addr),
                        timestamp: Utc::now(),
                    }],
                    affected_process: None,
                    timestamp: Utc::now(),
                    mitre_technique: Some("T1043".to_string()), // Commonly used port
                });
            }
        }
        None
    }

    pub fn detect_unusual_outbound(&self, conn: &NetworkConnection) -> Option<DetectionResult> {
        let name = conn.process_name.to_lowercase();
        
        if let Some(remote_addr) = conn.remote_addr {
            let port = remote_addr.port();
            
            if (name == "svchost.exe" || name == "lsass.exe") && port != 80 && port != 443 && port != 53 && port != 88 && port != 389 && port != 445 {
                return Some(DetectionResult {
                    id: Uuid::new_v4(),
                    rule_name: "Unusual Outbound System Process".to_string(),
                    threat_level: ThreatLevel::High,
                    confidence: 0.7,
                    description: format!("System process {} connected to unusual port {}", name, port),
                    evidence: vec![Evidence {
                        source: "network_monitor".to_string(),
                        detail: format!("Remote address: {}", remote_addr),
                        timestamp: Utc::now(),
                    }],
                    affected_process: None,
                    timestamp: Utc::now(),
                    mitre_technique: Some("T1571".to_string()), // Non-Standard Port
                });
            }
        }
        None
    }

    pub fn detect_high_connection_rate(connections: &[NetworkConnection], threshold: usize) -> Vec<DetectionResult> {
        let mut process_conn_count: HashMap<u32, usize> = HashMap::new();
        
        for conn in connections {
            *process_conn_count.entry(conn.pid).or_insert(0) += 1;
        }

        let mut results = Vec::new();
        for (pid, count) in process_conn_count {
            if count > threshold {
                results.push(DetectionResult {
                    id: Uuid::new_v4(),
                    rule_name: "High Connection Rate".to_string(),
                    threat_level: ThreatLevel::Medium,
                    confidence: 0.6,
                    description: format!("Process {} has {} active connections (threshold: {})", pid, count, threshold),
                    evidence: vec![Evidence {
                        source: "network_monitor".to_string(),
                        detail: format!("Connection count: {}", count),
                        timestamp: Utc::now(),
                    }],
                    affected_process: None,
                    timestamp: Utc::now(),
                    mitre_technique: Some("T1049".to_string()), // System Network Connections Discovery
                });
            }
        }
        results
    }
}
