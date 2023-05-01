use std::{collections::HashMap, ops::{AddAssign, MulAssign}, str::SplitWhitespace};

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
    Substruct(u32)
}

pub struct GameData {
    values_dict: HashMap<String,GlobalObject>
}

pub fn str_to_key_code(str: &str) -> KeyCode{
    match str {
        "A" => KeyCode::A,
        "B" => KeyCode::B,
        "C" => KeyCode::C,
        "D" => KeyCode::D,
        "E" => KeyCode::E,
        "F" => KeyCode::F,
        "G" => KeyCode::G,
        "H" => KeyCode::H,
        "I" => KeyCode::I,
        "J" => KeyCode::J,
        "K" => KeyCode::K,
        "L" => KeyCode::L,
        "M" => KeyCode::M,
        "N" => KeyCode::N,
        "O" => KeyCode::O,
        "P" => KeyCode::P,
        "Q" => KeyCode::Q,
        "R" => KeyCode::R,
        "S" => KeyCode::S,
        "T" => KeyCode::T,
        "U" => KeyCode::U,
        "V" => KeyCode::V,
        "W" => KeyCode::W,
        "X" => KeyCode::X,
        "Y" => KeyCode::Y,
        "Z" => KeyCode::Z,
        _ => panic!("invalid input!")
    }
}

fn get_lines_by_indentation_level(lines: &Vec<String> , start: usize, indentation_level: usize) -> (usize,usize){
    let end_index = lines[start..].iter()
        .position(|line| line.starts_with(&"\t".repeat(indentation_level)));
    if let Some(end_index) = end_index {
        return (start.clone() , end_index);
    }
    (start.clone(), lines.len())
}

fn parse_set_press(mut words_after_press_keyword : SplitWhitespace){
    if let Some("=>") = words_after_press_keyword.next() { }
    else {panic!("invalid input!");}

    if let Some(char) = words_after_press_keyword.next() {
        let key_code = str_to_key_code(char);
     }

}

pub fn parse(file_content: String){
    let lines = file_content.lines();
    let mut lines_vec : Vec<String> = vec![];
    for line in lines {
        lines_vec.push(line.to_string());
    }
    for (pos,line) in lines_vec.iter().enumerate() {
        let mut words = line.split_whitespace();
        match words.next() {
            None => {continue;},
            Some("set") => {},
            Some("setBoard") => {},
            Some("setEvent") => {
                let (start_scope, end_scope) = get_lines_by_indentation_level(&lines_vec, pos+1, 1);
            },
            Some("setPress") => {parse_set_press(words)},
            _ => {}
        }

    }
}
