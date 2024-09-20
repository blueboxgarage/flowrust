use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessDefinition {
    pub id: String,
    pub states: Vec<StateDefinition>,
    pub transitions: Vec<TransitionDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateDefinition {
    pub id: String,
    pub state_type: StateType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StateType {
    Start,
    End,
    Task,
    Decision,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionDefinition {
    pub from_state: String,
    pub to_state: String,
    pub condition: Option<String>,
}
