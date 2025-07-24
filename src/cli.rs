/// This module is responsible for collecting the CLI args and converting to intended types+values of args for use in other modules.

use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;

use std::io::{Seek, SeekFrom};
use std::io::prelude::*;

use std::env;
use crate::{cli::env::ArgsOs, task::{Task}};
use std::ffi::OsString;


/// HELPER FNS

// Converts OsString type to String
fn osstring_to_string(osstr:OsString) -> String {
    osstr.to_string_lossy().into_owned()
}

/// save_task_to_json: writes a serialized task to the JSON file, making sure to format the file properly. 
/// 
/// Args: 
///     task_serialized(String): String object of task that has been already turned into a JSON snippet, ready to be inserted
///     file(File): the opened File object (the JSON file) that is being written to.
/// 
/// Returns: 
///     N/A
/// 
pub fn save_task_to_json(task_serialized: String, mut file: File) {
    // seek to the end minus one to check for closing bracket
    file.seek(SeekFrom::End(-1)).expect("Seek failed.");

    let mut last_byte = [0u8; 1];
    file.read_exact(&mut last_byte).expect("Failed to read last byte.");

    // chop off the closing `]`
    file.set_len(file.metadata().unwrap().len() - 1).expect("Failed to truncate.");

    if last_byte[0] != b'[' && last_byte[0] != b'\n' {
        file.write_all(b",\n").expect("Failed to write comma.");
    }

    file.write_all(task_serialized.as_bytes()).expect("Failed to write task.");
    file.write_all(b"\n]").expect("Failed to close JSON array.");
}


/// create_json: creates a JSON file with name provided by fname.
/// 
/// Args: 
///     fname(&str): name of file to be created in root of project.
///     
/// 
/// Returns: 
///     File object that is opened, with read and append permissions.
/// 
pub fn create_json(fname: &str) -> File {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(fname)
        .expect("Failed to create/open database file.");

    // write opening array bracket if newly created
    let metadata = file.metadata().expect("Failed to get file metadata.");
    if metadata.len() == 0 {
        file.write_all(b"[\n").expect("Failed to write opening bracket.");
    }

    file
}


/// END HELPER FNS


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
///     parsed_args (Vec<String>): list where each element is a parsed arguments.
///                                 will always contain 2 elements for add functionality.
/// 
/// # Returns: Result<(), String>
/// 
/// TODO: append to JSON instead of overwriting.
pub fn command_add(parsed_args: Vec<String>) -> Result<(), String>{
    let mut success: bool = false;
    let fname: &'static str = "database.json";
    let json_file: File;    

    // create if task name exists already. otherwise, just open.
    // potential refactor , maybe just get rid of create_json() and put logic here.

    if !Path::new(fname).exists() {
        json_file = create_json(fname);
    } else {
        json_file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(fname).unwrap();
    }

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
    save_task_to_json(json, json_file);
    
    success = true;

    

    if success{
        Ok(())
    } else {
        Err("File was not created.".to_string())
    }
}








// #[cfg(test)]
// mod test{
//     use super::*;

//     #[test]
//     fn normal_command(){
//         // TODO: write tests with OsString types to simulate real cmds
//     }

// }