use shared_models::{RemediationAction, RemediationMode, Timestamp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionReviewCard {
    pub title: String,
    pub mode: RemediationMode,
    pub actions: Vec<RemediationAction>,
    pub queued_at: Timestamp,
}

impl ActionReviewCard {
    pub fn new(
        title: impl Into<String>,
        mode: RemediationMode,
        actions: Vec<RemediationAction>,
        queued_at: Timestamp,
    ) -> Self {
        Self {
            title: title.into(),
            mode,
            actions,
            queued_at,
        }
    }
}
