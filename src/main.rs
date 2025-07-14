// use crate::task::Task;
use crate::cli::parse_args;

mod task;
mod cli;

fn main(){

    // let name:String = args[1].clone();
    // let id:u16 = 0;
    //let task1 = Task{name:name, id:id};
    
    parse_args();

}