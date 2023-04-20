use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LevelFile {
    pub name: String,
    pub tilemap: Vec<Vec<u32>>,
    pub player: (usize, usize),
    pub citizens: Vec<(usize, usize)>,
    //pub policemen: Vec<(usize, usize)>
}

pub fn load_level(path: String) -> LevelFile {
    println!("loading {path}!");
    let source = std::fs::read_to_string(path).expect("Failed to read level file");
    let parsed_level: LevelFile = serde_json::from_str(&source).expect("Failed to parse level file");
    return parsed_level;
}
