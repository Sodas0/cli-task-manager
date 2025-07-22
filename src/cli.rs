/// This module is responsible for collecting the CLI args and converting to intended types+values of args for use in other modules.

use std::env;
use crate::cli::env::ArgsOs;
use std::ffi::OsString;


// Converts OsString type to String
fn osstring_to_string(osstr:OsString) -> String {
    osstr.to_string_lossy().into_owned()
}


/// Types of args:
///     1. -add <task_name>
///     2. -remove <task_name>
///     3. -view    
///     4. -done <task_name> 
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

    return parsed_list;

}



// #[cfg(test)]
// mod test{
//     use super::*;

//     #[test]
//     fn normal_command(){
//         // TODO: write tests with OsString types to simulate real cmds
//     }

// }