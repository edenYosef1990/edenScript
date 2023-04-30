
pub struct GameData;

fn get_lines_by_indentation_level(lines: &Vec<String> , start: usize, indentation_level: usize) -> (usize,usize){
    let end_index = lines[start..].iter()
        .position(|line| line.starts_with(&"\t".repeat(indentation_level)));
    if let Some(end_index) = end_index {
        return (start.clone() , end_index);
    }
    (start.clone(), lines.len())
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
            Some("press") => {},
            _ => {}
        }

    }
}
