#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    None,
    Floor,
    Wall
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; 1]
        }
    }
}
