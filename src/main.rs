// use crate::task::Task;
use crate::cli::{parse_args, command_add};

mod task;
mod cli;

fn main(){

    // let name:String = args[1].clone();
    // let id:u16 = 0;
    //let task1 = Task{name:name, id:id};
    
    let parsed_args: Vec<String> = parse_args();
    command_add(parsed_args).unwrap(); // unwrap more elegantly 
    

}

