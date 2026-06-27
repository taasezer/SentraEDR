use crate::metrics::{EtwIngestionReport, EtwIngestionStats};
use crate::normalize::normalize_etw_record;
use crate::source::EtwEventSource;
use shared_ipc::BoundedSender;
use shared_models::NormalizedTelemetryEvent;

pub struct EtwIngestor<S> {
    source: S,
    sender: BoundedSender<NormalizedTelemetryEvent>,
}

impl<S> EtwIngestor<S>
where
    S: EtwEventSource,
{
    pub fn new(source: S, sender: BoundedSender<NormalizedTelemetryEvent>) -> Self {
        Self { source, sender }
    }

    pub fn drain(mut self) -> EtwIngestionReport {
        let mut stats = EtwIngestionStats::default();

        loop {
            let record = match self.source.next_record() {
                Ok(Some(record)) => record,
                Ok(None) => break,
                Err(_) => {
                    stats.failed += 1;
                    break;
                }
            };

            stats.received += 1;
            let event = normalize_etw_record(record);
            match self.sender.try_send(event) {
                Ok(()) => stats.normalized += 1,
                Err(_) => stats.dropped += 1,
            }
        }

        EtwIngestionReport::new(stats, self.sender.health())
    }
}
