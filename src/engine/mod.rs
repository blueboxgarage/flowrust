// src/engine/mod.rs

pub mod process_definition;
pub mod state_machine;
pub mod workbasket;

pub use process_definition::ProcessDefinition;
pub use state_machine::ProcessInstance;

/// Loads a process definition from a JSON file.
pub fn load_process_definition(file_path: &str) -> ProcessDefinition {
    use serde_json;
    use std::fs;

    let data = fs::read_to_string(file_path).expect("Unable to read file");
    serde_json::from_str(&data).expect("JSON was not well-formatted")
}
