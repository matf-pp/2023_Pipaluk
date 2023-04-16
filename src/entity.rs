use crate::map::TileType;

extern crate queues;
use queues::*;

pub trait Search {
    fn get_position(&self) -> (usize, usize);
    fn find_shortest_path(&self, end: (usize, usize), map: &Vec<Vec<TileType>>) -> Vec<(usize, usize)> {
        let mut visited: Vec<Vec<bool>> = vec![];
        let mut parent: Vec<Vec<(isize, isize)>> = vec![];
        for i in 0..map.len() {
            visited.push(vec![]);
            parent.push(vec![]);
            for _ in 0..map[i as usize].len() {
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
        
        // let dx: Vec<isize> = vec![1, 0, -1, 0, 1, -1, 1, -1];
        // let dy: Vec<isize> = vec![0, 1, 0, -1, 1, -1, -1, 1];
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
                    
                    if x >= 0 && x < map.len() as isize {
                        if y >= 0 && y < map[x as usize].len() as isize {
                            let (x, y) = (x as usize, y as usize);
                            if !visited[x][y] && map[x][y] != TileType::Wall
                            && map[x][y] != TileType::None {
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
    }}

#[derive(Debug)]
pub struct Entity {
    pos: (usize, usize)
}

impl Entity {
    pub fn init(pos: (usize, usize)) -> Self {
        Self {pos}
    } 
}

impl Search for Entity {
    fn get_position(&self) -> (usize, usize) { self.pos }
}