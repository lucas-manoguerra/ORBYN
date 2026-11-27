use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub run_id: Uuid,
    pub node_id: Uuid,
    pub capability_envelope_id: Uuid,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionOutput {
    pub output: serde_json::Value,
    pub usage: Usage,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub estimated_cost_micros: u64,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("La ejecución fue cancelada")]
    Cancelled,
    #[error("Error del proveedor: {0}")]
    Provider(String),
    #[error("La política denegó la ejecución: {0}")]
    PolicyDenied(String),
}

#[async_trait]
pub trait AgentExecutor: Send + Sync {
    async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionOutput, RuntimeError>;
}

#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn provider_id(&self) -> &'static str;
    async fn complete(&self, model: &str, input: serde_json::Value) -> Result<serde_json::Value, RuntimeError>;
}

#[async_trait]
pub trait ToolProvider: Send + Sync {
    fn tool_id(&self) -> &'static str;
    async fn invoke(&self, input: serde_json::Value) -> Result<serde_json::Value, RuntimeError>;
}
