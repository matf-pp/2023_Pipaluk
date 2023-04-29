// these robots move around randomly when they don't see you
// when they see you, police will try to ctach you,  but will stop immediately when they lose sight of you

extern crate queues;
use queues::*;

use crate::entity::*;
use crate::{map::TileType, level::State};
use crate::robots::citizen::*;

extern crate rand;
use rand::Rng;

#[derive(Clone)]
pub struct Policeman {
    pos: (usize, usize)
}

impl Policeman {
    pub fn init(pos: (usize, usize)) -> Self {
        Self {pos}
    } 
    
    pub fn turn(&mut self, state: &State) -> Vec<(usize, usize)> {
        let player_pos = state.player.get_position();
        let citizens = &state.citizens;
        
        // if I see player, chase!
        let sees_player = self.sees(player_pos, &state.tilemap.tiles);
        if sees_player {
            println!(" Apprehending suspect!");
            let player_pos = state.player.get_position();
            let mut path = self.find_shortest_path(player_pos, state);
            unsafe { path.set_len(Self::SPEED.min(path.len())) };
            return path;
        }

        // if I hear a citizen plead for help, assist! (if multiple, chose closest)
        let panic_citizens = citizens
            .into_iter()
            .filter(|&citizen| citizen.mode == CitizenState::PANIC)
            .cloned()
            .collect::<Vec<Citizen>>();
        if panic_citizens.len() != 0 {
            println!(" Assisting citizen!");
            let closest = panic_citizens.iter().min_by_key(|k| k.distance_to(self.get_position()) as i32).unwrap();
            let mut path = self.find_shortest_path(closest.get_position(), state);
            unsafe { path.set_len(Self::SPEED.min(path.len())) };
            return path;
        }

        // otherwise wander aimlessly...
        println!(" Patrolling.");
        for _ in 1..8 {
            let delta: Vec<(isize, isize)> = vec![(1,0), (-1,0), (0,1), (0,-1)];
            let i = rand::thread_rng().gen_range(0..=3);
            
            let x = (self.pos.0 as isize + delta[i].0) as usize;
            let y = (self.pos.1 as isize + delta[i].1) as usize;
            
            if state.tile_free((x, y)) {
                return vec![(x, y)];
            }
        }
        return vec![];
    }
}

impl Entity for Policeman {
    const SPEED: usize = 2;
    fn get_position(&self) -> (usize, usize) { self.pos }
    fn set_position(&mut self, tile: (usize, usize)) {
        self.pos = tile;
    }
}

impl Search for Policeman {
    fn find_shortest_path(&self, end: (usize, usize), state: &State) -> Vec<(usize, usize)> {
        let mut visited: Vec<Vec<bool>> = vec![];
        let mut parent: Vec<Vec<(isize, isize)>> = vec![];
        for i in 0..state.tilemap.tiles.len() {
            visited.push(vec![]);
            parent.push(vec![]);
            for _ in 0..state.tilemap.tiles[i as usize].len() {
                visited[i].push(false);
                parent[i].push((-1, -1)); 
            } 
        } 
        
        let mut q: Queue<((isize, isize), isize)> = Queue::new();
        let (row, col) = (
            self.get_position().0 as usize,
            self.get_position().1 as usize
        );
        q.add(((row as isize, col as isize), 0)).unwrap();
        
        visited[row][col] = true;
        for citizen in state.citizens.iter() {
            if citizen.get_position() != end {
                let (r, c) = citizen.get_position();
                visited[r][c] = true;
            }
        }
        
        for policeman in state.policemen.iter() {
            let (r, c) = policeman.get_position();
            visited[r][c] = true;
        }
        
        for commando in state.commandos.iter() {
            let (r, c) = commando.get_position();
            visited[r][c] = true;
        }
        
        
        //let dx: Vec<isize> = vec![1, 0, -1, 0, 1, -1, 1, -1];
        //let dy: Vec<isize> = vec![0, 1, 0, -1, 1, -1, -1, 1];
        let dx: Vec<isize> = vec![1, 0, -1, 0];
        let dy: Vec<isize> = vec![0, 1, 0, -1];
        
        let mut found = false;
        
        while q.size() as isize != 0 {
            if let Ok(curr) = q.peek() {
                q.remove().unwrap();
                
                for i in 0..dx.len() {
                    let x = (curr.0).0 + dx[i];
                    let y = (curr.0).1 + dy[i];
                    let d = curr.1 + 1;
                    
                    if x >= 0 && x < state.tilemap.tiles.len() as isize {
                        if y >= 0 && y < state.tilemap.tiles[x as usize].len() as isize {
                            let (x, y) = (x as usize, y as usize);
                            if !visited[x][y] && state.tilemap.tiles[x][y] == TileType::Floor {
                                q.add(((x as isize, y as isize), d)).unwrap();
                                visited[x][y] = true;
                                parent[x][y] = ((curr.0).0, (curr.0).1); 
                            
                                if (x,y) == end {found = true; break;}
                            }
                        }
                    }
                }
            }
        }
        
        if found {
            let mut it = end;
            let mut result: Vec<(usize, usize)> = vec![];
            while parent[it.0][it.1] != (-1, -1) {
                result.push(it);
                it = (
                    parent[it.0][it.1].0 as usize,
                    parent[it.0][it.1].1 as usize
                );
            }
            
            result.reverse();
            result
        }
        else {vec![]}
    }
}

impl Sight for Policeman {
    const VIEW_DISTANCE: usize = 3;
}
