use sdl2::render::{ WindowCanvas };

use crate::{player::Player, entity::Sight};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    None,
    Floor,
    Wall,
    Liquid
}

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    topmost: i32,
    bottommost: i32,
    leftmost: i32,
    rightmost: i32,
    desired_translation_x: i32,
    desired_translation_y: i32,
    pub scale: u32,
    pub translation_x: i32,
    pub translation_y: i32
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![TileType::None; 0]; 0],
            topmost: std::i32::MAX,
            bottommost: std::i32::MIN,
            leftmost: std::i32::MAX,
            rightmost: std::i32::MIN,
            desired_translation_x: 0,
            desired_translation_y: 0,
            scale: 1,
            translation_x: 0,
            translation_y: 0
        }
    }

    pub fn passable(&self, tile: (usize, usize)) -> bool {
        let (row, col) = tile;
        self.tiles[row][col] == TileType::Floor
    }
    
    // get tile pos relative to the top corner of 0,0 in art space
    pub fn get_tile_pos(&mut self, row: usize, col: usize) -> (i32, i32) {
        let x = - 14 - (row as i32)*14 + (col as i32)*14;
        let y = (row as i32)*7 + (col as i32)*7;
        (x, y)
    }

    // get tile row and column based on coordinates in art space
    pub fn get_tile_index(&mut self, x: i32, y: i32) -> (usize, usize) {
        let row = (2*y-x) / 28;
        let col = (2*y+x) / 28;
        (row as usize, col as usize)
    }

    // get tilemap dimensions in art space
    pub fn get_dimensions(&mut self) -> (u32, u32) {
        let x = self.rightmost - self.leftmost;
        let y = self.bottommost - self.topmost;
        (x as u32, y as u32)
    }

    pub fn calc_view(&mut self) {
        let x = self.desired_translation_x - self.translation_x;
        let y = self.desired_translation_y - self.translation_y;

        self.translation_x += (x as f32 * 0.1) as i32;
        self.translation_y += (y as f32 * 0.1) as i32;
    }

    pub fn calc_scale_translation_debug(&mut self, canvas: &mut WindowCanvas) -> u32 {
        let (canvas_x, canvas_y) = canvas.output_size().unwrap();
        let (map_x, map_y) = self.get_dimensions();

        self.scale = (canvas_x as f32 / map_x as f32).min(canvas_y as f32 / map_y as f32) as u32;
        self.scale = self.scale.max(1);

        self.calc_translation_debug(canvas);

        self.scale
    }

    pub fn calc_translation_debug(&mut self, canvas: &mut WindowCanvas) -> (i32, i32) {
        let (canvas_x, canvas_y) = canvas.output_size().unwrap();
        let (map_x, map_y) = self.get_dimensions();
        
        self.desired_translation_x = - self.leftmost * self.scale as i32 + (canvas_x as i32 - map_x as i32 * self.scale as i32) / 2;
        self.desired_translation_y = - self.topmost * self.scale as i32 + (canvas_y as i32 - map_y as i32 * self.scale as i32) / 2;
        
        (self.desired_translation_x, self.desired_translation_y)
    }

    pub fn calc_scale_translation(&mut self, canvas: &mut WindowCanvas, player_pos: (usize, usize)) -> u32 {
        let canvas_size = canvas.output_size().unwrap();
        let canvas_size = (canvas_size.0 as i32, canvas_size.1 as i32);
        let visible_size = (Player::VIEW_DISTANCE as i32 * 28 + 50, Player::VIEW_DISTANCE as i32 * 19 + 50);

        self.scale = (canvas_size.0 as f32 / visible_size.0 as f32).min(canvas_size.1 as f32 / visible_size.1 as f32) as u32;
        self.scale = self.scale.max(1);

        self.calc_translation(canvas, player_pos);

        self.scale
    }

    pub fn calc_translation(&mut self, canvas: &mut WindowCanvas, player_pos: (usize, usize)) -> (i32, i32) {
        let player_pos = self.get_tile_pos(player_pos.0, player_pos.1);
        let canvas_size = canvas.output_size().unwrap();
        let canvas_size = (canvas_size.0 as i32, canvas_size.1 as i32);

        self.desired_translation_x = (- player_pos.0) * self.scale as i32 + canvas_size.0 / 2;
        self.desired_translation_y = (- player_pos.1) * self.scale as i32 + canvas_size.1 / 2;
        
        (self.desired_translation_x, self.desired_translation_y)
    }

    // debug print tilemap to console
    pub fn print(&self) {
        for row in self.tiles.iter() {
            for cell in row.iter() {
                match *cell {
                    TileType::Floor  => { print!(".") }, 
                    TileType::Wall   => { print!("#") }, 
                    TileType::None   => { print!(" ") }, 
                    TileType::Liquid => { print!("~") }, 
                }
            }
            println!();
        }
    }

    // load tilemap from json array
    pub fn load(&mut self, tilemap: Vec<Vec<u32>>) {
        let longest_row = tilemap.iter().fold(0, |acc, row| acc.max(row.len()));
        self.tiles.push(vec![TileType::None; longest_row+2]);
        for row in 0..tilemap.len() {
            self.tiles.push(vec![TileType::None; 0]);
            self.tiles[row+1].push(TileType::None);
            for col in 0..tilemap[row].len() {
                match tilemap[row][col] {
                    0 => { self.tiles[row+1].push(TileType::None) }
                    1 => { self.tiles[row+1].push(TileType::Floor) }
                    2 => { self.tiles[row+1].push(TileType::Wall) },
                    3 => { self.tiles[row+1].push(TileType::Liquid) }
                    _ => {}
                }
                match tilemap[row][col] {
                    0 => {}
                    1 | 3 => {
                        let (x, y) = self.get_tile_pos(row+1, col+1);
                        if y < self.topmost { self.topmost = y; }
                        if y > self.bottommost { self.bottommost = y; }
                        if x < self.leftmost { self.leftmost = x; }
                        if x > self.rightmost { self.rightmost = x; }
                    },
                    2 => {
                        let (_, y) = self.get_tile_pos(row+1, col+1);
                        if y < self.topmost { self.topmost = y; }
                    }
                    _ => {}
                }
            }
            self.tiles[row+1].push(TileType::None);
        }
        self.tiles.push(vec![TileType::None; longest_row+2]);
        self.topmost -= 6;
        self.rightmost += 28;
        self.bottommost += 19 + 6;
        // println!("topmost={}", self.topmost);
        // println!("bottommost={}", self.bottommost);
        // println!("leftmost={}", self.leftmost);
        // println!("rightmost={}", self.rightmost);
    }
}
