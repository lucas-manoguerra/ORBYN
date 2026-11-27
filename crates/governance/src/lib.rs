use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLimits {
    pub max_concurrent_runs: Option<u32>,
    pub max_runs_per_day: Option<u32>,
    pub max_tokens_per_day: Option<u64>,
    pub max_cost_micros_per_month: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityEnvelope {
    pub id: Uuid,
    pub subject_id: Uuid,
    pub project_id: Uuid,
    pub allowed_tools: Vec<String>,
    pub allowed_models: Vec<String>,
    pub allow_shell: bool,
    pub allow_network: bool,
    pub limits: UsageLimits,
}

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("Capacidad no concedida: {0}")]
    CapabilityDenied(String),
}

pub fn authorize_tool(envelope: &CapabilityEnvelope, tool: &str) -> Result<(), PolicyError> {
    if envelope.allowed_tools.iter().any(|allowed| allowed == tool) {
        Ok(())
    } else {
        Err(PolicyError::CapabilityDenied(tool.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deniega_herramientas_no_autorizadas() {
        let envelope = CapabilityEnvelope {
            id: Uuid::new_v4(),
            subject_id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            allowed_tools: vec!["lectura".into()],
            allowed_models: vec![],
            allow_shell: false,
            allow_network: false,
            limits: UsageLimits {
                max_concurrent_runs: None,
                max_runs_per_day: None,
                max_tokens_per_day: None,
                max_cost_micros_per_month: None,
            },
        };
        assert!(authorize_tool(&envelope, "shell").is_err());
    }
}
