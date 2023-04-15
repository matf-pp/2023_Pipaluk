use crate::map::TileType;

extern crate queues;
use queues::*;

pub trait Search {
    fn find_shortest_path(&self, end: (isize, isize), map: &Vec<Vec<TileType>>) -> Vec<(isize, isize)>;
}

#[derive(Debug)]
pub struct Entity {
    pos: (isize, isize)
}

impl Entity {
    pub fn init(pos: (isize, isize)) -> Self {
        Self {pos}
    } 
}

impl Search for Entity {
    fn find_shortest_path(&self, end: (isize, isize), map: &Vec<Vec<TileType>>) -> Vec<(isize, isize)> {
        let mut visited: Vec<Vec<bool>> = vec![];
        let mut parent: Vec<Vec<(isize, isize)>> = vec![];
        for i in 0..map.len() {
            visited.push(vec![]);
            parent.push(vec![]);
            for _ in 0..map[0].len() {
                visited[i].push(false);
                parent[i].push((-1, -1)); 
            } 
        } 
        
        let mut q: Queue<((isize, isize), isize)> = Queue::new();
        q.add((self.pos, 0)).unwrap();
        
        visited[self.pos.0 as usize][self.pos.1 as usize] = true;
        
        let dx: Vec<isize> = vec![1, 0, -1, 0, 1, -1, 1, -1];
        let dy: Vec<isize> = vec![0, 1, 0, -1, 1, -1, -1, 1];
        
        let mut found = false;
        
        while q.size() as isize != 0 {
            if let Ok(curr) = q.peek() {
                q.remove().unwrap();
                
                for i in 0..dx.len() {
                    let x = (curr.0).0 + dx[i];
                    let y = (curr.0).1 + dy[i];
                    let d = curr.1 + 1;
                    
                    if x >= 0 && y >= 0 && x < map.len() as isize && y < map[0].len() as isize {
                        if !visited[x as usize][y as usize] && map[x as usize][y as usize] != TileType::Wall
                        && map[x as usize][y as usize] != TileType::None {
                            q.add(((x, y),d)).unwrap();
                            visited[x as usize][y as usize] = true;
                            parent[x as usize][y as usize] = ((curr.0).0, (curr.0).1); 
                        
                            if (x,y) == end {found = true; break;}
                        }
                    }
                }
            }
        }
        
        if found {
            let mut it = end;
            let mut result: Vec<(isize, isize)> = vec![];
            while parent[it.0 as usize][it.1 as usize] != (-1, -1) {
                result.push(it);
                it = parent[it.0 as usize][it.1 as usize];
            }
            
            result.reverse();
            result
        }
        else {vec![]}
    }
}
