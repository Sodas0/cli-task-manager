mod task;
mod cli;
mod storage;


use crate::cli::{parse_args, command_add};
use crate::storage::{create_json};

use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

fn main(){
    
    //move the database file creation and opening here so that the opened file can be passed anywhere
    let fname: &'static str = "database.json";
    let json_file:File;


    // potential refactor , maybe just get rid of create_json() and put logic here.

    if !Path::new(fname).exists() {
        json_file = create_json(fname);
    } else {
        json_file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(fname).unwrap();
    }

    // parse arguments
    let parsed_args: Vec<String> = parse_args();
    
    command_add(parsed_args, json_file);

    //need controller here that will direct to correct command based on result of parse_args()
    // use parsed_args to determine what command was requested.

    
    // command_add(parsed_args).unwrap(); // unwrap more elegantly 
    

}

