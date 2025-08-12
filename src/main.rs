mod task;
mod cli;
mod storage;


use crate::cli::{parse_args, command_add, command_view, command_done, command_remove};
use crate::storage::{create_json};

use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

fn main(){
    
    //move the database file creation and opening here so that the opened file can be passed anywhere
    let fpath: &'static str = "database.json";
    let json_file:File;


    // potential refactor , maybe just get rid of create_json() and put logic here.

    if !Path::new(fpath).exists() {
        json_file = create_json(fpath);
    } else {
        json_file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(fpath).unwrap();
    }

    // parse arguments
    let parsed_args: Vec<String> = parse_args();
    
    
    // controller that will direct to the correct command

    let command = &parsed_args[0];

    match command.as_str() {
        "-add" => {
            if parsed_args.len() < 2 {
                eprintln!("Usage: -add <task_name>");
            } else {
                command_add(parsed_args, json_file);
            }
        }
        "-view" => {
            command_view(json_file).unwrap();
        }
        "-done" => {
            if parsed_args.len() < 2 {
                eprintln!("Usage: -done <task_name>");
            } else {
                if let Err(e) = command_done(parsed_args, fpath) {
                    eprintln!("Error: {}", e);
                }
            }
        }
        "-remove" => {
            if parsed_args.len() < 2 {
                eprintln!("Usage: -remove <task_name>");
            } else {
                if let Err(e) = command_remove(parsed_args, fpath) {
                    eprintln!("Error: {}", e);
                }
            }
        }
        _ => {
            eprintln!("Invalid command. Use -help for available options.");
        }
    }
    
    // command_add(parsed_args).unwrap(); // unwrap more elegantly 
    

}

