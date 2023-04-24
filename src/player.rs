use crate::entity::*;

#[derive(Clone)]
pub struct Player {
    pub pos: (usize, usize),
    pub current_sprite: String,
    pub flipped: bool
}

impl Player {
    pub fn init(pos: (usize, usize)) -> Self {
        Self {
            pos: pos, 
            current_sprite: "cat_idle_1".to_string(),
            flipped: false
        }
    } 
}

impl Entity for Player {
    fn get_position(&self) -> (usize, usize) { self.pos }
    fn set_position(&mut self, tile: (usize, usize)) {
        self.pos = tile;
    }
}

impl Search for Player {}

impl Sight for Player {
    const DISTANCE: usize = 7;
}