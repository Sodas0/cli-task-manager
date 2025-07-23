use serde::ser::{Serialize, Serializer, SerializeStruct};



// #[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    pub name: String, 
    pub id: u16,
    pub is_done: bool,    
}
impl Serialize for Task {
    fn serialize<S>(&self, serializer:S) -> Result<S::Ok, S::Error>
    where 
        S: Serializer,
        {
            let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("Task", 3)?;
            state.serialize_field("name", &self.name)?;
            state.serialize_field("id", &self.id)?;
            state.serialize_field("is_done", &self.is_done)?;
            state.end()
        }
}

