use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub name: String, 
    pub id: u16,
    pub is_done: bool,    
}

