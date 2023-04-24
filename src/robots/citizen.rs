// these robots move around randomly when they don't see you
// when they see you, they stand still and scream, alerting other robots (sight distance increase)

use crate::entity::*;
use crate::level::State;

extern crate rand;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitizenState { CALM, PANIC }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Citizen {
    pos: (usize, usize),
    pub mode: CitizenState
}

impl Citizen {
    pub fn init(pos: (usize, usize), mode: CitizenState) -> Self {
        Self {pos, mode}
    } 
    
    pub fn turn(&mut self, state: &State) -> Vec<(usize, usize)> {
        let player_pos = state.player.get_position();
        let sees = self.sees(player_pos, &state.tilemap.tiles);
        
        match sees {
            true => {
                println!("AAAAAA!");
                self.mode = CitizenState::PANIC;
                return vec![];
            },
            false => {
                println!("Calm...");
                self.mode = CitizenState::CALM;
                loop {
                    let delta: Vec<(isize, isize)> = vec![(1,0), (-1,0), (0,1), (0,-1)];
                    let i = rand::thread_rng().gen_range(0..3);
                    
                    let x = (self.pos.0 as isize + delta[i].0) as usize;
                    let y = (self.pos.1 as isize + delta[i].1) as usize;
                    
                    if state.tile_free((x, y)) {
                        return vec![(x, y)];
                    }
                }
            }
        }
    }
}

impl Entity for Citizen {
    fn get_position(&self) -> (usize, usize) { self.pos }
    fn set_position(&mut self, tile: (usize, usize)) {
        self.pos = tile;
    }
}

impl Search for Citizen {}

impl Sight for Citizen {
    const DISTANCE: usize = 2;
}


