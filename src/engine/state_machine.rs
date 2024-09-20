// src/engine/state_machine.rs

use super::process_definition::{
    ProcessDefinition, StateDefinition, StateType, TransitionDefinition,
};
use crate::engine::workbasket::{Task, WorkbasketManager};
use crate::utils::errors::EngineError;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInstance {
    pub id: String,
    pub definition: ProcessDefinition,
    pub current_state: String,
    pub variables: HashMap<String, String>,
}

impl ProcessInstance {
    pub fn new(definition: ProcessDefinition) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            current_state: "start".to_string(),
            definition,
            variables: HashMap::new(),
        }
    }

    pub fn execute(
        &mut self,
        workbasket_manager: &mut WorkbasketManager,
    ) -> Result<(), EngineError> {
        while self.current_state != "end" && self.current_state != "technical_hold" {
            let state = self.get_current_state()?;
            match state.state_type {
                StateType::Task => {
                    // Add task to workbasket and simulate task completion
                    info!("Adding task '{}' to workbasket", state.id);
                    self.execute_task(workbasket_manager)?;
                    if let Err(e) = self.complete_task(workbasket_manager) {
                        // Handle task failure
                        return Err(e);
                    }
                }
                StateType::Decision => {
                    // Evaluate condition and decide next state
                    self.evaluate_decision()?;
                }
                StateType::Start | StateType::End => {
                    // Move to next state
                    self.move_to_next_state()?;
                }
            }
        }
        Ok(())
    }

    fn get_current_state(&self) -> Result<&StateDefinition, EngineError> {
        self.definition
            .states
            .iter()
            .find(|s| s.id == self.current_state)
            .ok_or_else(|| EngineError::StateNotFound(self.current_state.clone()))
    }

    fn move_to_next_state(&mut self) -> Result<(), EngineError> {
        let transitions: Vec<&TransitionDefinition> = self
            .definition
            .transitions
            .iter()
            .filter(|t| t.from_state == self.current_state)
            .collect();

        if transitions.is_empty() {
            return Err(EngineError::TransitionNotFound(self.current_state.clone()));
        }

        let transition = transitions[0];
        info!(
            "Transitioning from '{}' to '{}'",
            self.current_state, transition.to_state
        );
        self.current_state = transition.to_state.clone();
        Ok(())
    }

    fn complete_task(
        &mut self,
        workbasket_manager: &mut WorkbasketManager,
    ) -> Result<(), EngineError> {
        info!("Completing task '{}'", self.current_state);

        // Simulate a task failure for "task_two"
        if self.current_state == "task_two" {
            // Task fails
            info!(
                "Task '{}' failed, routing to 'technical_hold' workbasket",
                self.current_state
            );

            // Create a failed task and add it to the 'technical_hold' workbasket
            let failed_task = Task::new(self.current_state.clone(), self.id.clone());
            workbasket_manager.add_task_to_workbasket("technical_hold", failed_task);

            // Update the current state to reflect the failure
            self.current_state = "technical_hold".to_string();

            return Err(EngineError::TaskFailed(self.current_state.clone()));
        } else {
            // Task succeeds, move to the next state
            self.move_to_next_state()
        }
    }

    fn evaluate_decision(&mut self) -> Result<(), EngineError> {
        self.move_to_next_state()
    }

    fn execute_task(
        &mut self,
        workbasket_manager: &mut WorkbasketManager,
    ) -> Result<(), EngineError> {
        let state = self.get_current_state()?;
        let task = Task::new(state.id.clone(), self.id.clone());
        workbasket_manager.add_task_to_workbasket("default", task);
        Ok(())
    }
}
