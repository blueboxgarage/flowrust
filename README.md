# go2-rust

`go2-rust` is a Rust-based workflow engine inspired by the Flowret project.
It allows you to define and execute business processes using state machines and workbasket routing.
The main goal of this project is to experiment with rust-language and seek performance benefits.
## Features

- **Process Definitions**: Define processes in JSON format.
- **State Machine Execution**: Execute processes based on defined states and transitions.
- **Workbasket Routing**: Assign tasks to workbaskets for processing.
- **Persistence**: Store process instances and tasks using SQLite.
- **RESTful APIs**: Interact with the engine via HTTP endpoints.

## Getting Started

### Prerequisites

- Rust (version 1.56 or higher)
- SQLite

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/go2-rust.git
   cd go2-rust
