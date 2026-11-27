use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub id: Uuid,
    pub name: String,
    pub version: u64,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: Uuid,
    pub name: String,
    pub kind: NodeKind,
    #[serde(default)]
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Trigger,
    Agent,
    Model,
    Tool,
    Condition,
    Parallel,
    Join,
    HumanApproval,
    SubWorkflow { workflow_id: Uuid, version: u64 },
    Output,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub from: Uuid,
    pub to: Uuid,
    pub condition: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("La orquestación contiene identificadores de nodo duplicados")]
    DuplicateNode,
    #[error("Una conexión referencia un nodo inexistente")]
    MissingNode,
    #[error("La orquestación contiene un ciclo no admitido")]
    Cycle,
}

impl WorkflowDefinition {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut ids = HashSet::new();
        for node in &self.nodes {
            if !ids.insert(node.id) {
                return Err(ValidationError::DuplicateNode);
            }
        }

        let mut indegree: HashMap<Uuid, usize> =
            ids.iter().map(|id| (*id, 0)).collect();
        let mut outgoing: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        for edge in &self.edges {
            if !ids.contains(&edge.from) || !ids.contains(&edge.to) {
                return Err(ValidationError::MissingNode);
            }
            *indegree.get_mut(&edge.to).ok_or(ValidationError::MissingNode)? += 1;
            outgoing.entry(edge.from).or_default().push(edge.to);
        }

        let mut ready: Vec<Uuid> = indegree
            .iter()
            .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
            .collect();
        let mut visited = 0usize;

        while let Some(id) = ready.pop() {
            visited += 1;
            if let Some(next) = outgoing.get(&id) {
                for target in next {
                    let degree = indegree.get_mut(target).ok_or(ValidationError::MissingNode)?;
                    *degree -= 1;
                    if *degree == 0 {
                        ready.push(*target);
                    }
                }
            }
        }

        if visited != ids.len() {
            return Err(ValidationError::Cycle);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rechaza_conexiones_a_nodos_inexistentes() {
        let workflow = WorkflowDefinition {
            id: Uuid::new_v4(),
            name: "inválida".into(),
            version: 1,
            nodes: vec![],
            edges: vec![WorkflowEdge {
                from: Uuid::new_v4(),
                to: Uuid::new_v4(),
                condition: None,
            }],
        };
        assert_eq!(workflow.validate(), Err(ValidationError::MissingNode));
    }
}
