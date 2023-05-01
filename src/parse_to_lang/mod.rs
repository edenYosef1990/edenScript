use core::panic;
use std::{collections::HashMap, ops::{AddAssign, MulAssign}, str::SplitWhitespace};

use bevy::{prelude::KeyCode, transform::commands};

use self::{types::{GameData, EventSource, Event, Command, SetPropertyCommandInfo, Op}, entity_types::EntityCreation};
mod types;
mod entity_types;

pub fn line_to_create_entity(str: &String) -> EntityCreation {
    let mut words = str.split(",");
    let mut first_part = words.next().unwrap().split_whitespace();
    first_part.next();
    let name = first_part.next().unwrap();
    let mut entity = EntityCreation::new(name.to_string());

    while let Some(word) = words.next() {
        let mut property_tuple = word.split_whitespace();
        let property_name = property_tuple.next().unwrap();
        if let Some(value_str) = property_tuple.next() {
            entity.properties.push(entity_types::PropertyCreation::PropertyWithValue(property_name.to_string(), value_str.to_string()));
        }
        else {
            entity.properties.push(entity_types::PropertyCreation::Label(property_name.to_string()));
        }
    }
    entity

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
        return (start.clone() , end_index + start);
    }
    (start.clone(), lines.len())
}

fn line_to_command(str: &String) -> Command {
    let mut words = str.split_whitespace();
    let word = words.next().unwrap();

    let mut words_inside_word = word.split(".");
    let entity_name = words_inside_word.next().unwrap();
    let property_name = if let Some(property_name) = words_inside_word.next() {property_name} else {"position"};

    println!("property is {}",property_name);
    let op = words.next().unwrap();
    let value :u32 = words.next().unwrap().trim().parse().unwrap();
    Command::SetPropertyCommand(SetPropertyCommandInfo{
        object_id: entity_name.to_string(),
        property_name: property_name.to_string(),
        op: match op {
            "+=" => Op::Add(value),
            "-=" => Op::Substruct(value),
            "=" => Op::AssignVal(value),
            _ => panic!("invalid input!")
        }
    })
}

fn compose_event_object(event_source: EventSource , lines: &[String]) -> Event {
    let commands : Vec<Command> = lines.iter().map(|line| line_to_command(line)).collect();
    Event { event_source: event_source , invoked_commands: commands }
}

fn parse_set_press(mut words_after_press_keyword : SplitWhitespace) -> EventSource{
    if let Some(char) = words_after_press_keyword.next() {
        let key_code = str_to_key_code(char);

        if let Some("=>") = words_after_press_keyword.next() { }
        else {panic!("invalid input!");}

        return EventSource::KeyPressed(key_code);
     }
     panic!("invalid input!");
}

pub fn parse(file_content: String){
    let mut game_Data = GameData::new();
    let lines = file_content.lines();
    let mut lines_vec : Vec<String> = vec![];
    for line in lines {
        lines_vec.push(line.to_string());
    }
    for (pos,line) in lines_vec.iter().enumerate() {
        let mut words = line.split_whitespace();
        match words.next() {
            None => {continue;},
            Some("setEntity") => {
                game_Data.entities_creation.push(line_to_create_entity(line));
            },
            Some("setBoard") => {},
            Some("setEvent") => {
                let (start_scope, end_scope) = get_lines_by_indentation_level(&lines_vec, pos+1, 1);
            },
            Some("onPress") => {
                let (start_scope, end_scope) = get_lines_by_indentation_level(&lines_vec, pos+1, 1);
                let event_source = parse_set_press(words);
                let event = compose_event_object(event_source, &lines_vec[start_scope..end_scope]);
                game_Data.events.push(event);
            },
            _ => {}
        }

    }
    println!("{:?}",game_Data);
}
