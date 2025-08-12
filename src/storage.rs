
use std::io::{Seek, SeekFrom, Result as IoResult};
use std::io::prelude::*;
use std::ffi::OsString;

use std::fs::File;
use std::fs::OpenOptions;
use crate::task::Task;
use serde_json;

// HELPER FNS

// Converts OsString type to String
pub fn osstring_to_string(osstr:OsString) -> String {
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
pub fn save_task_to_json(task_serialized: String, mut file: File) -> IoResult<()>{
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

    Ok(())
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

/// read_tasks_from_json: reads and deserializes all tasks from the JSON file
/// 
/// Args: 
///     file_path(&str): path to the JSON file
/// 
/// Returns: 
///     Result<Vec<Task>, String> - Vector of tasks or error message
/// 
pub fn read_tasks_from_json(file_path: &str) -> std::result::Result<Vec<Task>, String> {
    let file = File::open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    let reader = std::io::BufReader::new(file);
    
    // Try to deserialize as array of tasks
    match serde_json::from_reader::<_, Vec<Task>>(reader) {
        Ok(tasks) => Ok(tasks),
        Err(e) => {
            // If it's an empty file or malformed, return empty vector
            if e.is_eof() || e.is_data() {
                Ok(Vec::new())
            } else {
                Err(format!("Failed to parse JSON: {}", e))
            }
        }
    }
}

/// write_tasks_to_json: writes a vector of tasks to the JSON file, overwriting existing content
/// 
/// Args: 
///     tasks(Vec<Task>): vector of tasks to write
///     file_path(&str): path to the JSON file
/// 
/// Returns: 
///     Result<(), String> - success or error message
/// 
pub fn write_tasks_to_json(tasks: Vec<Task>, file_path: &str) -> std::result::Result<(), String> {
    let json_string = serde_json::to_string_pretty(&tasks)
        .map_err(|e| format!("Failed to serialize tasks: {}", e))?;
    
    std::fs::write(file_path, json_string)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(())
}

// END HELPER FNS