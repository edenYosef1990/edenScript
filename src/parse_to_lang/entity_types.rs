#[derive(Debug)]
pub enum PropertyCreation{
    PropertyWithValue(String,String),
    Label(String)
}
#[derive(Debug)]
pub struct EntityCreation {
    pub name : String,
    pub properties: Vec<PropertyCreation>
}

impl EntityCreation {
    pub fn new(name: String) -> Self {
        Self { name: name , properties: vec![] }
    }
}
