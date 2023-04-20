// these robots move around randomly when they don't see you
// when they see you, police will try to ctach you,  but will stop immediately when they lose sight of you

use crate::entity::*;
use crate::level::State;
use crate::robots::citizen::*;

extern crate rand;
use rand::Rng;

pub struct Policeman {
    pos: (usize, usize),
    speed: usize
}

impl Policeman {
    pub fn init(pos: (usize, usize), speed: usize) -> Self {
        Self {pos, speed}
    } 
    
    pub fn turn(&mut self, _state: &State) {
        let player_pos = _state.player.get_position();
        let citizens = &_state.citizens;
        
        //go to nearest citizen that's panicking (if there is one)
        let panic_citizens = citizens
            .into_iter()
            .filter(|citizen| citizen.mode == CitizenState::PANIC)
            .cloned()
            .collect::<Vec<Citizen>>();
        
        if panic_citizens.len() != 0 {
            let path = self.find_shortest_path(panic_citizens[0].get_position(), &_state.tilemap.tiles);
            if self.speed <= path.len() {
                self.pos = path[self.speed];
            }
        }
        
        let sees_player = self.sees(player_pos, &_state.tilemap.tiles);
        match sees_player {
            //go to player
            true => {
                println!("I see you!");
                let player_pos = _state.player.get_position();
                let path = self.find_shortest_path(player_pos, &_state.tilemap.tiles);
                
                if self.speed <= path.len() {
                    self.pos = path[self.speed];
                }
            }
            //random movement
            false => {
                let dx: Vec<isize> = vec![1, -1, 0, 0];
                let dy: Vec<isize> = vec![0, 0, -1, 1];
                
                let x = self.pos.0 + dx[rand::thread_rng().gen_range(0..dx.len())] as usize;
                let y = self.pos.1 + dy[rand::thread_rng().gen_range(0..dy.len())] as usize;
                
                self.pos = (self.speed*x, self.speed*y);
            }
        }
    }
}

impl Entity for Policeman {
    fn get_position(&self) -> (usize, usize) { self.pos }
}

impl Search for Policeman {}

impl Sight for Policeman {
    const DISTANCE: usize = 5;
}
