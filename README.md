**CLI TASK MANAGER**

**[WIP]**


A simple task application that you can run in the terminal.

This project is a toy project/exercise to improve my understanding of Rust and CLI app workflows.
    


Installation instructions:

```bash
git clone https://Sodas0/cli-task-manager
cd cli-task-manager
```
Install dependencies & compile project:
```bash
cargo build --release
```
Run CLI:
```bash
cargo run -- <command> <args>
```

Examples (Future TODO: need to be able to account for spaces in tasks.): 
```bash
cargo run -- -add "Write_task" 
cargo run -- -view
cargo run -- -done "Write_task"
```



[Current status]

- Arg parser functional (Need to define how data flows depending on detected command)

- Persistent task storage completed

- Command to add tasks functional

[TODO]

- Complete other commands

- Be able to run commands without having to type ```cargo run -- <command> <args>``` every time

