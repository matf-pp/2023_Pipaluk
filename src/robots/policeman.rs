// these robots move around randomly when they don't see you
// when they see you, police will try to ctach you,  but will stop immediately when they lose sight of you

use crate::entity::*;
use crate::level::State;

pub struct Policeman {
    pos: (usize, usize),
}

impl Policeman {
    pub fn init(pos: (usize, usize)) -> Self {
        Self {pos}
    } 
    
    pub fn turn(&mut self, _state: &State) {
        let player_pos = _state.player.get_position();
        let sees = self.sees(player_pos, &_state.tilemap.tiles);
        
        match sees {
            true => todo!(), //needs to be specified!!!
            false => {}
        }
    }
}

impl Entity for Policeman {
    fn get_position(&self) -> (usize, usize) { self.pos }
}

impl Search for Policeman {}

impl Sight for Policeman {
    const DISTANCE: usize = 3;
}
