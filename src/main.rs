// src/main.rs

mod engine;
mod utils;

use crate::engine::workbasket::WorkbasketManager;
use engine::load_process_definition;
use engine::ProcessInstance;
use utils::logger::init_logger;

fn main() {
    // Initialize the logger
    init_logger();

    // Load the sample journey process definition
    let process_definition = load_process_definition("examples/sample_journey.json");

    // Create a new process instance
    let mut process_instance = ProcessInstance::new(process_definition);

    // Create a new workbasket manager
    let mut workbasket_manager = WorkbasketManager::new();

    // Execute the process instance
    match process_instance.execute(&mut workbasket_manager) {
        Ok(_) => {
            println!("Process execution completed successfully.");
        }
        Err(e) => {
            eprintln!("Process execution encountered an error: {}", e);
        }
    }

    // Create the output folder if it doesn't exist
    std::fs::create_dir_all("output").expect("Failed to create output directory");

    // Write the process instance state to a file
    let process_instance_file = format!("output/process_instance_{}.json", process_instance.id);
    let process_instance_json = serde_json::to_string_pretty(&process_instance)
        .expect("Failed to serialize process instance");
    std::fs::write(&process_instance_file, process_instance_json)
        .expect("Failed to write process instance file");

    // Write the workbasket tasks to a file
    let workbaskets_file = "output/workbaskets.json";
    let workbaskets_json =
        serde_json::to_string_pretty(&workbasket_manager).expect("Failed to serialize workbaskets");
    std::fs::write(workbaskets_file, workbaskets_json).expect("Failed to write workbaskets file");

    println!("Outputs are stored in the 'output' directory.");
}
