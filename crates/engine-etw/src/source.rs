use crate::error::EtwError;
use crate::record::EtwRecord;
use std::collections::VecDeque;

pub trait EtwEventSource {
    fn next_record(&mut self) -> Result<Option<EtwRecord>, EtwError>;
}

#[derive(Debug, Clone)]
pub struct SyntheticEtwSource {
    records: VecDeque<EtwRecord>,
}

impl SyntheticEtwSource {
    pub fn from_records(records: impl IntoIterator<Item = EtwRecord>) -> Self {
        Self {
            records: records.into_iter().collect(),
        }
    }
}

impl EtwEventSource for SyntheticEtwSource {
    fn next_record(&mut self) -> Result<Option<EtwRecord>, EtwError> {
        Ok(self.records.pop_front())
    }
}
