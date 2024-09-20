# flowrust

`flowrust` is a Rust-based workflow engine inspired by the unify-flowret project. (https://github.com/blueboxgarage/unify-flowret)

It allows you to define and execute business processes using state machines and workbasket routing.
The main goal of this project is to experiment with rust-language and seek performance benefits in open-source bpm.
## Features

- **Process Definitions**: Define processes in JSON format.
- **State Machine Execution**: Execute processes based on defined states and transitions.
- **Workbasket Routing**: Assign tasks to workbaskets for processing.
- **Persistence**: Store process instances and tasks using SQLite.
- **RESTful APIs**: Interact with the engine via HTTP endpoints.

### Prerequisites

- Rust (version 1.56 or higher)
- SQLite

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/go2-rust.git
   cd go2-rust

## Getting Started 
Setting Up and Running the go2-rust Program

This guide will help you set up and run the go2-rust program, which simulates a workflow engine and demonstrates how tasks can fail and be routed to different workbaskets.
Prerequisites

Before you begin, ensure you have the following installed on your system:

    Rust and Cargo: Install Rust and its package manager, Cargo, by following the instructions at rustup.rs.
    Git: (Optional) If you need to clone the repository from a Git server.

Project Structure

The project directory should look like this:

lua

go2-rust/
├── Cargo.toml
├── Cargo.lock
├── examples
│   └── sample_journey.json
├── output
│   └── (Generated output files will be stored here)
├── src
    ├── main.rs
    ├── engine
    │   ├── mod.rs
    │   ├── process_definition.rs
    │   ├── state_machine.rs
    │   └── workbasket.rs
    └── utils
        ├── mod.rs
        ├── errors.rs
        └── logger.rs

Setup Instructions
1. Clone the Repository

If you haven't already, clone the repository to your local machine:

bash

git clone https://github.com/yourusername/go2-rust.git
cd go2-rust

Replace yourusername with your actual GitHub username or the appropriate URL if the repository is hosted elsewhere.
2. Install Rust and Cargo

If Rust and Cargo are not installed, install them by running:

bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Follow the on-screen instructions to complete the installation.
3. Build the Project

Compile the project using Cargo:

bash

cargo build

This command will download and compile all dependencies specified in Cargo.toml and build the project.
4. Run the Program

Run the program using Cargo:

bash

cargo run

5. Observe the Output

After running the program, you should see output similar to:

arduino

Process execution encountered an error: Task failed at state: technical_hold
Outputs are stored in the 'output' directory.

6. Check the Output Files

The program generates output files in the output directory:

    Process Instance State: output/process_instance_<process_id>.json
    Workbaskets State: output/workbaskets.json

a. Process Instance Output

This file contains the state of the process instance after execution.

Example Content:

json

{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "definition": {
    "id": "sample_journey",
    "states": [
      { "id": "start", "state_type": "Start" },
      { "id": "task_one", "state_type": "Task" },
      { "id": "task_two", "state_type": "Task" },
      { "id": "end", "state_type": "End" }
    ],
    "transitions": [
      { "from_state": "start", "to_state": "task_one", "condition": null },
      { "from_state": "task_one", "to_state": "task_two", "condition": null },
      { "from_state": "task_two", "to_state": "end", "condition": null }
    ]
  },
  "current_state": "technical_hold",
  "variables": {}
}

b. Workbaskets Output

This file contains the tasks assigned to different workbaskets.

Example Content:

json

{
  "workbaskets": {
    "default": {
      "id": "default",
      "tasks": [
        {
          "id": "4a94d4c2-f0ae-4e3b-bf33-f09b1b6d1c6a",
          "state_id": "task_one",
          "process_instance_id": "123e4567-e89b-12d3-a456-426614174000"
        },
        {
          "id": "b3e0d2a6-3b9c-4e1d-bc7d-5d87d6f8b0d2",
          "state_id": "task_two",
          "process_instance_id": "123e4567-e89b-12d3-a456-426614174000"
        }
      ]
    },
    "technical_hold": {
      "id": "technical_hold",
      "tasks": [
        {
          "id": "e5d6a6d4-6c7b-4f8d-bd7d-9c0e6a5d1b2f",
          "state_id": "task_two",
          "process_instance_id": "123e4567-e89b-12d3-a456-426614174000"
        }
      ]
    }
  }
}

7. Understanding the Output

    Process Instance State: Indicates that the process is currently in the "technical_hold" state due to a simulated task failure.
    Workbaskets:
        default: Contains tasks that were processed normally.
        technical_hold: Contains the task that failed ("task_two"), which was routed here for further action.

8. Customizing the Process

You can modify the sample_journey.json file located in the examples directory to customize the process flow.

Example Changes:

    Add more tasks or states.
    Introduce decision points.
    Modify transitions to create loops or alternative paths.

9. Re-running the Program

After making changes to the process definition or the code:

    Rebuild the Project:

    bash

cargo build

Run the Program Again:

bash

    cargo run

10. Troubleshooting

    Compilation Errors: Ensure all dependencies are correctly specified in Cargo.toml and that the code is free of syntax errors.
    Runtime Errors: Check the console output for error messages and stack traces.
    Missing Output Files: Ensure the output directory exists or that the program has permissions to create and write to it.

Dependencies

The project relies on the following Rust crates:

    serde: For serialization and deserialization.
    serde_json: To work with JSON data.
    uuid: For generating unique identifiers.
    log and env_logger: For logging.
    thiserror: For error handling.

These dependencies are specified in the Cargo.toml file and are automatically downloaded and compiled when you build the project.
Contributing

Contributions are welcome! If you'd like to contribute:

    Fork the Repository: Create a personal copy of the repository on your GitHub account.

    Create a New Branch: For your feature or bug fix.

    bash

git checkout -b feature/your-feature-name

Make Changes: Implement your feature or fix.

Commit Your Changes:

bash

git commit -am "Add new feature"

Push to Your Fork:

bash

    git push origin feature/your-feature-name

    Submit a Pull Request: From your fork's branch to the main repository.

License

This project is licensed under the MIT License. See the LICENSE file for details.
