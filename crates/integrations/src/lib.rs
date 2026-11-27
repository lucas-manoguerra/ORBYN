use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryRef {
    pub provider: String,
    pub owner: String,
    pub name: String,
    pub identity_connection_id: String,
}

#[derive(Debug, Error)]
pub enum IntegrationError {
    #[error("Se requiere autenticación")]
    AuthenticationRequired,
    #[error("Permiso denegado")]
    PermissionDenied,
    #[error("Error del proveedor: {0}")]
    Provider(String),
}

#[async_trait]
pub trait SourceControlProvider: Send + Sync {
    async fn list_branches(&self, repository: &RepositoryRef) -> Result<Vec<String>, IntegrationError>;
    async fn current_revision(&self, repository: &RepositoryRef) -> Result<Option<String>, IntegrationError>;
}
