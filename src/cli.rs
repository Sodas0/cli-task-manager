/// This module is responsible for collecting the CLI args and converting to intended types+values of args for use in other modules.

use std::path::Path;
use std::fs::File;

use std::io::prelude::*;

use std::env;
use crate::{cli::env::ArgsOs, task::{Task}};
use std::ffi::OsString;


// Converts OsString type to String
fn osstring_to_string(osstr:OsString) -> String {
    osstr.to_string_lossy().into_owned()
}



/// parse_args()
/// #  Args: 
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




/// command_add():
///     Creates a new task and stores it in the local JSON database.
///        
/// 
/// # Args: 
///     parsed_args (Vec<String>): list where each element is a parsed arguments.
///                                 will always contain 2 elements for add functionality.
/// 
/// # Returns: Result<(), String>
/// 
/// 
pub fn command_add(parsed_args: Vec<String>) -> Result<(), String>{
    let mut success: bool = false;
    let fname = "database.json";
    // need to check if task name exists already - panic if it does
    //      in the future, maybe work on recover
    
    let task_name: String = parsed_args[1].clone();
    let task_id: u16 = 0; // use random ID later.
    let task_is_done: bool = false;

    // create a task instance with the specified name
    let task = Task{
        name: task_name,
        id: task_id,
        is_done: task_is_done,
    };

    let json = serde_json::to_string(&task).unwrap();
    save_task_to_json(json, fname);

    // if fname exists in dir, sucess = true
    if Path::new(fname).exists() {
        success = true;
    }

    if success{
        Ok(())
    } else {
        Err("File was not created.".to_string())
    }
}


pub fn save_task_to_json(task_serialized:String, fname: &str){
  
    // need checks here to see if task already exists. 
    let mut file = File::create(fname).expect("something went wrong buddy");
    file.write_all(task_serialized.as_bytes()).expect("couldnt create file");

}



// #[cfg(test)]
// mod test{
//     use super::*;

//     #[test]
//     fn normal_command(){
//         // TODO: write tests with OsString types to simulate real cmds
//     }

// }