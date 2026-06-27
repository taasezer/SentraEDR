use crate::error::EtwError;
use crate::record::EtwProcessRecord;
use std::collections::VecDeque;

pub trait EtwEventSource {
    fn next_record(&mut self) -> Result<Option<EtwProcessRecord>, EtwError>;
}

#[derive(Debug, Clone)]
pub struct SyntheticEtwSource {
    records: VecDeque<EtwProcessRecord>,
}

impl SyntheticEtwSource {
    pub fn from_records(records: impl IntoIterator<Item = EtwProcessRecord>) -> Self {
        Self {
            records: records.into_iter().collect(),
        }
    }
}

impl EtwEventSource for SyntheticEtwSource {
    fn next_record(&mut self) -> Result<Option<EtwProcessRecord>, EtwError> {
        Ok(self.records.pop_front())
    }
}
