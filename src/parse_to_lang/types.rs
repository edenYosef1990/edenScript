use std::{collections::HashMap, ops::{MulAssign, AddAssign}};

use bevy::prelude::KeyCode;

pub struct PropertiesGroup{
    pub components_names: Vec<String>
}

// it works given 99 components in total in game
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

pub struct GlobalObject {
    properties_dict: HashMap<String,u32>
}

pub enum Command {
    SetPropertyCommand(SetPropertyCommandInfo),
    CreateObject,
    RemoveObject
}

pub struct SetPropertyCommandInfo{
    object_id: String,
    property_name: String,
    op: Op
}

pub enum Op {
    Add(u32),
    Substruct(u32),
    AssignVal(u32)
}

pub enum EventSource{
    MouseClickedOnScreen(usize,usize),
    KeyPressed(KeyCode)
}

pub struct Event {
    event_source: EventSource,
    invoked_commands: Vec<Command>
}

pub struct GameData {
    values_dict: HashMap<String,GlobalObject>,
    events: Vec<Event>
}