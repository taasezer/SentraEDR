use crate::{
    BoundedReceiver, BoundedSender, IpcEnvelope, IpcError, IpcMessageKind, bounded_channel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcDispatcherConfig {
    pub queue_capacity: usize,
}

impl IpcDispatcherConfig {
    pub fn try_new(queue_capacity: usize) -> Result<Self, IpcError> {
        if queue_capacity == 0 {
            return Err(IpcError::InvalidDispatcherCapacity {
                capacity: queue_capacity,
            });
        }

        Ok(Self { queue_capacity })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IpcRouteStats {
    pub accepted: u64,
    pub rejected: u64,
    pub dropped: u64,
}

impl IpcRouteStats {
    fn record_accept(&mut self) {
        self.accepted += 1;
    }

    fn record_reject(&mut self) {
        self.rejected += 1;
    }

    fn record_drop(&mut self) {
        self.dropped += 1;
    }
}

#[derive(Debug)]
struct Route {
    sender: BoundedSender<IpcEnvelope>,
    stats: IpcRouteStats,
}

impl Route {
    fn new(name: &str, capacity: usize) -> (Self, BoundedReceiver<IpcEnvelope>) {
        let (sender, receiver) = bounded_channel(name, capacity);
        (
            Self {
                sender,
                stats: IpcRouteStats::default(),
            },
            receiver,
        )
    }

    fn dispatch(&mut self, envelope: IpcEnvelope) -> Result<(), IpcError> {
        match self.sender.try_send(envelope) {
            Ok(()) => {
                self.stats.record_accept();
                Ok(())
            }
            Err(error) => {
                if matches!(error, IpcError::QueueFull { .. }) {
                    self.stats.record_drop();
                }
                Err(error)
            }
        }
    }
}

#[derive(Debug)]
pub struct IpcDispatcher {
    health_route: Route,
    telemetry_summary_route: Route,
    alert_route: Route,
    user_decision_route: Route,
    remediation_request_route: Route,
    remediation_status_route: Route,
    audit_record_route: Route,
    rejected: u64,
    pub health: BoundedReceiver<IpcEnvelope>,
    pub telemetry_summaries: BoundedReceiver<IpcEnvelope>,
    pub alerts: BoundedReceiver<IpcEnvelope>,
    pub user_decisions: BoundedReceiver<IpcEnvelope>,
    pub remediation_requests: BoundedReceiver<IpcEnvelope>,
    pub remediation_statuses: BoundedReceiver<IpcEnvelope>,
    pub audit_records: BoundedReceiver<IpcEnvelope>,
}

impl IpcDispatcher {
    pub fn new(config: IpcDispatcherConfig) -> Self {
        let (health_route, health) = Route::new("ipc-health", config.queue_capacity);
        let (telemetry_summary_route, telemetry_summaries) =
            Route::new("ipc-telemetry-summary", config.queue_capacity);
        let (alert_route, alerts) = Route::new("ipc-alert", config.queue_capacity);
        let (user_decision_route, user_decisions) =
            Route::new("ipc-user-decision", config.queue_capacity);
        let (remediation_request_route, remediation_requests) =
            Route::new("ipc-remediation-request", config.queue_capacity);
        let (remediation_status_route, remediation_statuses) =
            Route::new("ipc-remediation-status", config.queue_capacity);
        let (audit_record_route, audit_records) =
            Route::new("ipc-audit-record", config.queue_capacity);

        Self {
            health_route,
            telemetry_summary_route,
            alert_route,
            user_decision_route,
            remediation_request_route,
            remediation_status_route,
            audit_record_route,
            rejected: 0,
            health,
            telemetry_summaries,
            alerts,
            user_decisions,
            remediation_requests,
            remediation_statuses,
            audit_records,
        }
    }

    pub fn dispatch(&mut self, envelope: IpcEnvelope) -> Result<(), IpcError> {
        if let Err(error) = envelope.validate() {
            self.rejected += 1;
            self.route_stats_mut(envelope.kind).record_reject();
            return Err(error);
        }

        self.route_mut(envelope.kind).dispatch(envelope)
    }

    pub fn rejected_count(&self) -> u64 {
        self.rejected
    }

    pub fn alert_stats(&self) -> IpcRouteStats {
        self.alert_route.stats
    }

    pub fn remediation_request_stats(&self) -> IpcRouteStats {
        self.remediation_request_route.stats
    }

    fn route_mut(&mut self, kind: IpcMessageKind) -> &mut Route {
        match kind {
            IpcMessageKind::Health => &mut self.health_route,
            IpcMessageKind::TelemetrySummary => &mut self.telemetry_summary_route,
            IpcMessageKind::Alert => &mut self.alert_route,
            IpcMessageKind::UserDecision => &mut self.user_decision_route,
            IpcMessageKind::RemediationRequest => &mut self.remediation_request_route,
            IpcMessageKind::RemediationStatus => &mut self.remediation_status_route,
            IpcMessageKind::AuditRecord => &mut self.audit_record_route,
        }
    }

    fn route_stats_mut(&mut self, kind: IpcMessageKind) -> &mut IpcRouteStats {
        &mut self.route_mut(kind).stats
    }
}
