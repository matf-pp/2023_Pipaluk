use sdl2::render::{ WindowCanvas };

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    None,
    Floor,
    Wall
}

pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    topmost: i32,
    bottommost: i32,
    leftmost: i32,
    rightmost: i32,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![TileType::None; 0]; 0],
            topmost: std::i32::MAX,
            bottommost: std::i32::MIN,
            leftmost: std::i32::MAX,
            rightmost: std::i32::MIN,
        }
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

    // get scale and translation vector needed to center and fill canvas with tilemap
    pub fn get_scale_and_translation(&mut self, canvas: &mut WindowCanvas) -> (u32, (i32, i32)) {
        let (canvas_x, canvas_y) = canvas.output_size().unwrap();
        let (map_x, map_y) = self.get_dimensions();
        
        let scale = (canvas_x as f32 / map_x as f32).min(canvas_y as f32 / map_y as f32) as u32;
        let translation_x = - self.leftmost * scale as i32 + (canvas_x - map_x * scale) as i32 / 2;
        let translation_y = (canvas_y - map_y * scale) as i32 / 2;

        (scale, (translation_x, translation_y))
    }

    // debug print tilemap to console
    pub fn _print(&mut self) {
        for row in self.tiles.iter() {
            for cell in row.iter() {
                print!("{}", *cell as i32);
            }
            println!();
        }
    }

    // load tilemap from json array
    pub fn load(&mut self, tilemap: Vec<Vec<u32>>) {
        for row in 0..tilemap.len() {
            self.tiles.push(vec![TileType::None; 0]);
            for col in 0..tilemap[row].len() {
                match tilemap[row][col] {
                    0 => { self.tiles[row].push(TileType::None) }
                    1 => { self.tiles[row].push(TileType::Floor) }
                    2 => { self.tiles[row].push(TileType::Wall) }
                    _ => {}
                }
                match tilemap[row][col] {
                    0 => {}
                    _ => {
                        let (x, y) = self.get_tile_pos(row, col);
                        if y < self.topmost { self.topmost = y; }
                        if y > self.bottommost { self.bottommost = y; }
                        if x < self.leftmost { self.leftmost = x; }
                        if x > self.rightmost { self.rightmost = x; }
                    }
                }
            }
        }
        self.rightmost += 28 + 1;
        self.bottommost += 19 + 1;
        // println!("topmost={}", self.topmost);
        // println!("bottommost={}", self.bottommost);
        // println!("leftmost={}", self.leftmost);
        // println!("rightmost={}", self.rightmost);
    }
}
