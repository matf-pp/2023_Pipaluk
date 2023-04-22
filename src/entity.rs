use crate::{map::TileType, level::State};

extern crate queues;
use queues::*;

pub trait Entity {
    fn get_position(&self) -> (usize, usize);
}

pub trait Search: Entity {
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
            let (r, c) = citizen.get_position();
            visited[r][c] = true;
        }
        for policeman in state.policemen.iter() {
            let (r, c) = policeman.get_position();
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

pub trait Sight: Entity {
    const DISTANCE: usize;
    fn sees(&self, target: (usize, usize), _map: &Vec<Vec<TileType>>) -> bool {
        let (self_row, self_col) = self.get_position();
        let (other_row, other_col) = target;

        self_row.abs_diff(other_row) + self_col.abs_diff(other_col) <= Self::DISTANCE
    }
}
