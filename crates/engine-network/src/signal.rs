use crate::analyzer::DestinationHistory;
use crate::event::{NetworkDirection, NetworkEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkSignal {
    pub name: String,
    pub description: String,
    pub severity: SignalSeverity,
    pub event: NetworkEvent,
}

pub fn signals_for_event(event: &NetworkEvent, history: &DestinationHistory) -> Vec<NetworkSignal> {
    let mut signals = Vec::new();

    if is_rare_external_destination(event) {
        signals.push(signal(
            "rare_external_destination",
            "Outbound connection targets a non-local destination outside the small benign list",
            SignalSeverity::Medium,
            event,
        ));
    }

    if has_suspicious_dns_pattern(event) {
        signals.push(signal(
            "suspicious_dns_pattern",
            "Domain metadata has suspicious structure",
            SignalSeverity::Medium,
            event,
        ));
    }

    if history.observation_count >= 3
        && history.last_interval_seconds.is_some()
        && history.last_interval_seconds == history.previous_interval_seconds
    {
        signals.push(signal(
            "beacon_interval_candidate",
            "Destination was observed at repeated equal synthetic intervals",
            SignalSeverity::High,
            event,
        ));
    }

    if matches!(event.remote_port, 4444 | 1337 | 6667 | 31337) {
        signals.push(signal(
            "high_risk_port",
            "Remote port is in the initial high-risk port list",
            SignalSeverity::Medium,
            event,
        ));
    }

    if event.direction == NetworkDirection::Outbound
        && event.domain.is_none()
        && !is_private_or_local_ip(&event.remote_ip)
    {
        signals.push(signal(
            "ip_literal_connection",
            "Outbound public IP connection has no domain metadata",
            SignalSeverity::Low,
            event,
        ));
    }

    signals
}

fn signal(
    name: &str,
    description: &str,
    severity: SignalSeverity,
    event: &NetworkEvent,
) -> NetworkSignal {
    NetworkSignal {
        name: name.to_string(),
        description: description.to_string(),
        severity,
        event: event.clone(),
    }
}

fn is_rare_external_destination(event: &NetworkEvent) -> bool {
    if event.direction != NetworkDirection::Outbound || is_private_or_local_ip(&event.remote_ip) {
        return false;
    }

    !matches!(
        event.domain.as_deref().map(str::to_ascii_lowercase),
        Some(domain)
            if domain == "localhost"
                || domain.ends_with("microsoft.com")
                || domain.ends_with("windowsupdate.com")
    )
}

fn has_suspicious_dns_pattern(event: &NetworkEvent) -> bool {
    let Some(domain) = event.domain.as_deref() else {
        return false;
    };
    let domain = domain.to_ascii_lowercase();
    domain.len() > 80
        || domain.split('.').count() >= 5
        || domain.contains("duckdns")
        || domain.contains("no-ip")
        || domain.ends_with(".tk")
        || domain.ends_with(".top")
}

fn is_private_or_local_ip(ip: &str) -> bool {
    if ip == "::1" {
        return true;
    }
    let parts: Vec<u8> = ip
        .split('.')
        .filter_map(|part| part.parse::<u8>().ok())
        .collect();
    if parts.len() != 4 {
        return false;
    }
    parts[0] == 127
        || parts[0] == 10
        || (parts[0] == 172 && (16..=31).contains(&parts[1]))
        || (parts[0] == 192 && parts[1] == 168)
}
