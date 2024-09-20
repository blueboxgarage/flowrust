// src/engine/workbasket.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkbasketManager {
    pub workbaskets: HashMap<String, Workbasket>,
}

impl WorkbasketManager {
    pub fn new() -> Self {
        Self {
            workbaskets: HashMap::new(),
        }
    }

    pub fn add_task_to_workbasket(&mut self, workbasket_id: &str, task: Task) {
        let workbasket = self
            .workbaskets
            .entry(workbasket_id.to_string())
            .or_insert_with(|| Workbasket::new(workbasket_id));
        workbasket.tasks.push(task);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workbasket {
    pub id: String,
    pub tasks: Vec<Task>,
}

impl Workbasket {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            tasks: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub state_id: String,
    pub process_instance_id: String,
}

impl Task {
    pub fn new(state_id: String, process_instance_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            state_id,
            process_instance_id,
        }
    }
}
