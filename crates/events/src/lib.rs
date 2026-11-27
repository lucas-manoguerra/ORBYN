use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub event_id: Uuid,
    pub correlation_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub actor_id: Option<Uuid>,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(correlation_id: Uuid, actor_id: Option<Uuid>, payload: T) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            correlation_id,
            occurred_at: Utc::now(),
            actor_id,
            payload,
        }
    }
}
