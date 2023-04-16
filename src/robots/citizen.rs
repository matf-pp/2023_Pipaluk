// these robots move around randomly when they don't see you
// when they see you, they stand still and scream, alerting other robots (sight distance increase)

use crate::entity::*;
use crate::level::State;

pub struct Citizen {
    pos: (usize, usize),
}

impl Citizen {
    pub fn init(pos: (usize, usize)) -> Self {
        Self {pos}
    } 

    pub fn turn(&self, _state: &State) {
        // _state.tilemap._print();
    }
}

impl Entity for Citizen {
    fn get_position(&self) -> (usize, usize) { self.pos }
}

impl Search for Citizen {}

impl Sight for Citizen {
    const DISTANCE: usize = 3;
}


