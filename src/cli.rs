/// This module is responsible for collecting the CLI args and converting to intended types+values of args for use in other modules.

use std::fs::File;

use std::env;
use crate::{task::{Task}};
use crate::{cli::env::ArgsOs};
use std::io::BufReader;
use serde_json::Value;

use crate::storage::{osstring_to_string, save_task_to_json};



/// parse_args: When called, analyzes the command line arguments provided at runtime and returns a list of parsed arguments,
///                 ignoring the first one.
/// #  Args: 
///     N/A
/// 
/// # Returns: Vector of Strings where each element corresponds to a provided command line argument.
/// 
/// # Allowed arguments:
///
///     1. -add <task_name>
///     2. -remove <task_name>
///     3. -view    
///     4. -done <task_name> 
/// 
pub fn parse_args() -> Vec<String> {
    let args:ArgsOs = env::args_os();
    let mut parsed_list: Vec<String> = Vec::new();

    // convert each argument (OsString) into a String and store them individually in parsed_list.
    for argument in args.into_iter().skip(1){
        let arg: String = osstring_to_string(argument);
        parsed_list.push(arg);
    }

    // sanity checks for elements in parsed list.
    
    // check if args provided.
    if parsed_list.is_empty(){
        panic!("No enough arguments provided. Use -help for help.")
    }
    
    // we know by nature that the maximum length for a cmd is 2 elements.
    if parsed_list.len() > 2{
        panic!("Too many arguments provided. Use -help for help.")
    }

    // the first element is predictable, it is simply a command that we make.
    // i.e. first element should be in {-add, -remove, -view, -done}
    // TODO: create aliases and better checks
    let command_list: Vec<&'static str> = vec!["-add", "-remove", "-help", "-view", "-done"];
    let cmd: &str = &parsed_list[0];

    if !command_list.contains(&cmd){
        panic!("Not a valid command. Use -help for help.")
    }

    parsed_list

}




/// command_add: Creates a new task and stores it in the local JSON database.
///        
/// 
/// # Args: 
///     parsed_args (Vec<String>): list where each element is a parsed arguments. Will always contain 2 elements for add functionality.
///     json_file (File): opened JSON file that stores task information.
/// 
/// # Returns: Result<(), String>
/// 
/// TODO: append to JSON instead of overwriting.
pub fn command_add(parsed_args: Vec<String>, json_file: File){
    
    let task_name: String = parsed_args[1].clone();
    let task_id: u16 = 0; // TODO: use random ID later.
    let task_is_done: bool = false;

    // create a task instance with the specified name
    let task = Task{
        name: task_name,
        id: task_id,
        is_done: task_is_done,
    };

    let json = serde_json::to_string(&task).unwrap();
    save_task_to_json(json, json_file).unwrap();
    

}


/// command_view: prints out list of tasks from JSON.
///        
/// # Args: 
///     json_file(File): the opened JSON file that stores the task data
/// 
/// # Returns: Result<(), String>
/// 
/// TODO: Implement first. Make it print more neatly later.
pub fn command_view(json_file: File) -> Result<(), String>{
    
    let reader = BufReader::new(json_file);

    let data: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");

    if let Value::Array(items) = data {
        for (i, item) in items.iter().enumerate() {
            println!("Task {}: {}", i, item);
        }
    } else {
        println!("JSON is not an array");
    }
    
    Ok(())
}


/// command_done: marks a task as completed by setting its is_done field to true
///        
/// # Args: 
///     parsed_args (Vec<String>): list where each element is a parsed argument. Will always contain 2 elements for done functionality.
///     file_path (&str): path to the JSON file storing task information.
/// 
/// # Returns: Result<(), String>
/// 
pub fn command_done(parsed_args: Vec<String>, file_path: &str) -> Result<(), String> {
    let task_name: String = parsed_args[1].clone();
    
    // Read all tasks from the JSON file
    let mut tasks = crate::storage::read_tasks_from_json(file_path)?;
    
    // Find the task by name and mark it as done
    let mut task_found = false;
    for task in &mut tasks {
        if task.name == task_name {
            task.is_done = true;
            task_found = true;
            break;
        }
    }
    
    if !task_found {
        return Err(format!("Task '{}' not found", task_name));
    }
    
    // Write the updated tasks back to the file
    crate::storage::write_tasks_to_json(tasks, file_path)?;
    
    println!("Task '{}' marked as completed!", task_name);
    Ok(())
}


// #[cfg(test)]
// mod test{
//     use super::*;

//     #[test]
//     fn normal_command(){
//         // TODO: write tests with OsString types to simulate real cmds
//     }

// }