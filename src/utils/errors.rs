// src/utils/errors.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("State not found: {0}")]
    StateNotFound(String),
    #[error("Transition from state '{0}' not found")]
    TransitionNotFound(String),
    #[error("Task failed at state: {0}")]
    TaskFailed(String),
    // Add other error variants as needed
}
