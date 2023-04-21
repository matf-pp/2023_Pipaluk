// these robots move around randomly when they don't see you
// when they see you, police will try to ctach you,  but will stop immediately when they lose sight of you

use crate::entity::*;
use crate::level::State;
use crate::robots::citizen::*;

extern crate rand;
use rand::Rng;

#[derive(Clone)]
pub struct Policeman {
    pos: (usize, usize),
    speed: usize
}

impl Policeman {
    pub fn init(pos: (usize, usize), speed: usize) -> Self {
        Self {pos, speed}
    } 
    
    pub fn turn(&mut self, state: &State) {
        let player_pos = state.player.get_position();
        let citizens = &state.citizens;
        
        // if I see player, chase!
        let sees_player = self.sees(player_pos, &state.tilemap.tiles);
        if sees_player {
            println!("Apprehending suspect!");
            let player_pos = state.player.get_position();
            let path = self.find_shortest_path(player_pos, &state);
            
            if self.speed < path.len() {
                self.pos = path[self.speed];
            }
            else {
                self.pos = *path.last().unwrap();
            }
            return;
        }

        // if I hear a citizen plead for help, assist!
        let mut panic_citizens = citizens
            .into_iter()
            .filter(|&citizen| citizen.mode == CitizenState::PANIC)
            .cloned()
            .collect::<Vec<Citizen>>();
        // find closest civilian
        panic_citizens.sort_by(|x, y| x.cmp(y));
        
        if panic_citizens.len() != 0 {
            let citizen_pos = panic_citizens[0].get_position();
            let path = self.find_shortest_path(citizen_pos, &state);
            //debug
            println!("Path length: {}", path.len());
            for c in panic_citizens {
                println!("{:?}", c);
            }
            
            if path.len() == 0 {return;}
            
            if self.speed < path.len() {
                self.pos = path[self.speed];
            }
            else {
                self.pos = *path.last().unwrap();
            }
            return;
        }

        // otherwise wander aimlessly...
        println!("Patrolling.");
        loop {
            let delta: Vec<(isize, isize)> = vec![(1,0), (-1,0), (0,1), (0,-1)];
            let i = rand::thread_rng().gen_range(0..3);
            
            let x = (self.pos.0 as isize + delta[i].0) as usize;
            let y = (self.pos.1 as isize + delta[i].1) as usize;
            
            if state.tile_free((x, y)) {
                self.pos = (x, y);
                return;
            }
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
