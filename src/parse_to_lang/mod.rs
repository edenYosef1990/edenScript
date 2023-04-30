pub struct GameData;

fn parse(file_content: String){
    let lines = file_content.lines();
    for line in lines {
        let mut words = line.split_whitespace();
        match words.next() {
            None => {continue;},
            Some("set") => {},
            Some("setEvent") => {},
            Some("press") => {},
            _ => {}
        }
    }
}