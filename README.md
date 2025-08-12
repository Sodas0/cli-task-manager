**CLI TASK MANAGER**

A simple task application that you can run in the terminal.

This project is a toy project/exercise to improve my understanding of Rust and CLI app workflows.

## Installation

```bash
git clone https://Sodas0/cli-task-manager
cd cli-task-manager
```

Install dependencies & compile project:
```bash
cargo build --release
```

## Usage

Run CLI:
```bash
cargo run -- <command> <args>
```

### Available Commands

- `-add <task_name>` - Add a new task
- `-view` - View all tasks
- `-done <task_name>` - Mark a task as completed
- `-remove <task_name>` - Remove a task

### Examples

```bash
# Add tasks
cargo run -- -add "Write documentation"
cargo run -- -add "Fix bug in login"

# View all tasks
cargo run -- -view

# Mark task as completed
cargo run -- -done "Write documentation"

# Remove a task
cargo run -- -remove "Fix bug in login"
```

## Current Status

**Completed Features:**
- Argument parser with command validation
- Persistent JSON storage for tasks
- Add tasks command (`-add`)
- View tasks command (`-view`)
- Mark tasks as done (`-done`)
- Remove tasks command (`-remove`)
- Proper error handling and user feedback
- JSON serialization/deserialization with serde

## TODO

- [ ] Add `-help` command for usage instructions
- [ ] Improve view command formatting (currently shows raw JSON)
- [ ] Generate unique task IDs (currently hardcoded to 0)
- [ ] Create proper binary installation (so users don't need `cargo run --`)
- [ ] Add tests
- [ ] Better handling of spaces in task names (currently works with quotes)

