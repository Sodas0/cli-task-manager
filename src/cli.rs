/// This module is responsible for collecting the CLI args and converting to intended types+values of args for use in other modules.

use std::env;
use crate::cli::env::ArgsOs;

/// Types of args:
///     1. -add <task_name>
///     2. -remove <task_name>
///     3. -view    
///     4. -done <task_name> 
pub fn parse_args(){
    let args:ArgsOs = env::args_os();

    for argument in args{
        println!("{argument:?}")
    }
}


