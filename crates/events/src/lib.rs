use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T> {
    pub event_id: Uuid,
    pub correlation_id: Uuid,
    pub occurred_at_unix_ms: i64,
    pub actor_id: Option<Uuid>,
    pub payload: T,
}

impl<T> EventEnvelope<T> {
    pub fn new(
        correlation_id: Uuid,
        actor_id: Option<Uuid>,
        occurred_at_unix_ms: i64,
        payload: T,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            correlation_id,
            occurred_at_unix_ms,
            actor_id,
            payload,
        }
    }
}
