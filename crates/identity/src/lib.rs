use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub external_account_id: String,
    pub display_name: Option<String>,
    pub credential_ref: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
}
