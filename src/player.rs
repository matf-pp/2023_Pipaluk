use crate::entity::*;

pub struct Player {
    pos: (usize, usize),
}

impl Player {
    pub fn init(pos: (usize, usize)) -> Self {
        Self {pos}
    } 
}

impl Entity for Player {
    fn get_position(&self) -> (usize, usize) { self.pos }
}

impl Search for Player {}

impl Sight for Player {
    const DISTANCE: usize = 5;
}