use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn parse_rfc3339(value: &str) -> Result<Self, TimestampError> {
        let parsed = DateTime::parse_from_rfc3339(value)
            .map_err(|source| TimestampError::InvalidRfc3339 {
                value: value.to_owned(),
                source,
            })?
            .with_timezone(&Utc);
        Ok(Self(parsed))
    }

    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }
}

#[derive(Debug, Error)]
pub enum TimestampError {
    #[error("invalid RFC3339 timestamp '{value}'")]
    InvalidRfc3339 {
        value: String,
        source: chrono::ParseError,
    },
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn parses_rfc3339_as_utc() {
        let timestamp = Timestamp::parse_rfc3339("2026-06-27T12:00:00+03:00").unwrap();
        assert_eq!(timestamp.to_rfc3339(), "2026-06-27T09:00:00+00:00");
    }
}
