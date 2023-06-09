use crate::{map::TileType, level::State};

extern crate queues;
use queues::*;

pub trait Entity {
    const SPEED: usize;
    fn get_position(&self) -> (usize, usize);
    fn set_position(&mut self, tile: (usize, usize));
    fn distance_to(&self, other: (usize, usize)) -> f32 {
        let (self_row, self_col) = self.get_position();
        let (other_row, other_col) = other;
        let squared = (self_row as i32 - other_row as i32).pow(2) + (self_col as i32 - other_col as i32).pow(2);
        (squared as f32).sqrt()
    }
    fn get_vector(&self, other: (f32, f32)) -> (f32, f32){
        let(self_row, self_col) = self.get_position();
        let mut dir_row: f32 = other.0 - self_row as f32;
        let mut dir_col: f32 = other.1 - self_col as f32;
        let norm = (dir_row*dir_row + dir_col*dir_col).sqrt();
        dir_row /= norm;
        dir_col /= norm;

        return (dir_row, dir_col);
    }
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

pub trait Sight: Entity {
    const VIEW_DISTANCE: usize;
    fn sees(&self, target: (usize, usize), map: &Vec<Vec<TileType>>) -> bool {
        let (self_row, self_col) = self.get_position();
        let (other_row, other_col) = target;

        if self.distance_to(target) > Self::VIEW_DISTANCE as f32 {return false;}
        if (self_row, self_col) == (other_row, other_col) {return true;}
        
        let mut dir_col: f32 = other_col as f32 - self_col as f32;
        let mut dir_row: f32 = other_row as f32 - self_row as f32;
        let norm = (dir_row*dir_row + dir_col*dir_col).sqrt();
        dir_row /= norm;
        dir_col /= norm;

        let mut curr_row = self_row as f32;
        let mut curr_col = self_col as f32;

        while (dir_col >= 0.0 && curr_col.round() as usize <= other_col || 
               dir_col <= 0.0 && curr_col.round() as usize >= other_col) &&
              (dir_row >= 0.0 && curr_row.round() as usize <= other_row || 
               dir_row <= 0.0 && curr_row.round() as usize >= other_row){

            if map[curr_row.round() as usize][curr_col.round() as usize] == TileType::Wall{

                if map[other_row][other_col] != TileType::Wall{return false;}
                if (curr_row.round() as usize, curr_col.round() as usize) == target{return true;}

               //checks for angles smaller that 45deg
                let row_check: bool = ((curr_row.round() - 1.0) as usize, curr_col.round() as usize) == target
                || ((curr_row.round() + 1.0) as usize, curr_col.round() as usize) == target;

                let col_check: bool = (curr_row.round() as usize, (curr_col.round() - 1.0) as usize) == target
                || (curr_row.round() as usize, (curr_col.round() + 1.0) as usize) == target;
                
                let angle_check = (row_check && !col_check) || (col_check && !row_check);

                let mut floating_wall: bool = true;
                let neighbours = vec![
                    (other_row - 1, other_col - 1),(other_row - 1, other_col), 
                    (other_row - 1, other_col), (other_row, other_col - 1), 
                    (other_row, other_col + 1), (other_row + 1, other_col - 1),
                    (other_row + 1, other_col), (other_row + 1, other_col + 1)];
                
                //number of visible tiles that are not Wall
                let mut s: usize = 0;
                //number of visible corner iles
                let mut c: usize = 0;
                let mut i: usize = 0;
                for n in neighbours.into_iter(){
                    if n.0 < map.len() && n.1 < map[0].len() && n.0 >=1 && n.1 >=1
                    && map[n.0][n.1] != TileType::Wall && self.sees(n, map){
                        floating_wall = false;
                        s += 1;
                        if i == 1 || i == 3 || i == 4 || i == 6{c += 1;}
                    }
                    i += 1;
                }
              
                floating_wall |= s<=1;
                if s == c {floating_wall = true;}
                            
                return (angle_check && !floating_wall) || !floating_wall;
            
            }

            curr_row += dir_row;
            curr_col += dir_col;
        }

        return true;
    }

}
