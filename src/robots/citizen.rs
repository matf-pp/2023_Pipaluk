// these robots move around randomly when they don't see you
// when they see you, they stand still and scream, alerting other robots (sight distance increase)

use crate::entity::*;
use crate::level::State;

#[derive(Clone, Copy, PartialEq)]
pub enum CitizenState { CALM, PANIC }

#[derive(Clone)]
pub struct Citizen {
    pos: (usize, usize),
    pub mode: CitizenState
}

impl Citizen {
    pub fn init(pos: (usize, usize), mode: CitizenState) -> Self {
        Self {pos, mode}
    } 
    
    pub fn turn(&mut self, _state: &State) {
        let player_pos = _state.player.get_position();
        let sees = self.sees(player_pos, &_state.tilemap.tiles);
        
        match sees {
            true => {
                println!("AAAAAA!");
                self.mode = CitizenState::PANIC;
            },
            false => {
                println!("Calm...");
                self.mode = CitizenState::CALM;
            }
        }
    }
}

impl Entity for Citizen {
    fn get_position(&self) -> (usize, usize) { self.pos }
}

impl Search for Citizen {}

impl Sight for Citizen {
    const DISTANCE: usize = 2;
}


