use std::{collections::HashMap, ops::{MulAssign, AddAssign}};

use bevy::prelude::KeyCode;

use super::entity_types::EntityCreation;

#[derive(Debug)]
pub struct PropertiesGroup{
    pub components_names: Vec<String>
}

// it works given 99 components in total in game
#[derive(Debug)]
pub struct CompilerState{
    component_to_id_dict : HashMap<String,u32>,
    components_group_to_id: HashMap<u32,u32>
}

impl CompilerState {
    pub fn hash_array_size_n(mut array: Vec<u32>) -> u32{
        let mut index : u32 = 0;
        array.sort();
        for item in array {
            index.mul_assign(100);
            index.add_assign(item);
        }
        index
    }

    pub fn components_group_to_id(&self, components_group: &PropertiesGroup) -> u32{
        let components_ids : Vec<u32> = components_group.components_names.iter()
        .map(|component_name| self.component_to_id_dict.get(component_name).unwrap().clone()).collect();
        let group_id = CompilerState::hash_array_size_n(components_ids);
        self.components_group_to_id.get(&group_id).unwrap().clone()
    }
}

#[derive(Debug)]
pub struct GlobalObject {
    properties_dict: HashMap<String,u32>
}

#[derive(Debug)]
pub enum Command {
    SetPropertyCommand(SetPropertyCommandInfo),
    CreateObject,
    RemoveObject
}

#[derive(Debug)]
pub struct SetPropertyCommandInfo{
    pub object_id: String,
    pub property_name: String,
    pub op: Op
}

#[derive(Debug)]
pub enum Op {
    Add(u32),
    Substruct(u32),
    AssignVal(u32)
}

#[derive(Debug)]
pub enum EventSource{
    MouseClickedOnScreen(usize,usize),
    KeyPressed(KeyCode)
}

#[derive(Debug)]
pub struct Event {
    pub event_source: EventSource,
    pub invoked_commands: Vec<Command>
}

#[derive(Debug)]
pub struct Grid {
    pub id: String,
    pub layer_number: u32,
    pub image_path: String

}

#[derive(Debug)]
pub struct GameData {
    pub entities_creation: Vec<EntityCreation>,
    pub values_dict: HashMap<String,GlobalObject>,
    pub events: Vec<Event>,
    pub grids: Vec<Grid>
}

impl GameData{
    pub fn new() -> Self{
        Self { values_dict: HashMap::new() , events: vec![] , grids: vec![] , entities_creation: vec![] }
    }
}