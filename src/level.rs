use serde::{Deserialize, Serialize};
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator, BlendMode};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, SystemCursor};

use crate::map::{self, TileType};
use crate::entity::{Entity, Search};

extern crate sdl2;

#[derive(Serialize, Deserialize)]
struct LevelFile {
    name: String,
    tilemap: Vec<Vec<u32>>,
    player: (usize, usize)
}

fn load_level(path: String) -> LevelFile {
    println!("loading {path}!");
    let source = std::fs::read_to_string(path).expect("Failed to read level file");
    let parsed_level: LevelFile = serde_json::from_str(&source).expect("Failed to parse level file");
    return parsed_level;
}

#[derive(PartialEq)]
pub enum GameResult {
    Quit,
    _Victory,
    _Defeat
}

pub fn play_level(
    canvas: &mut WindowCanvas, 
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &mut EventPump,
    name: &str
) -> GameResult {
    let cursor = Cursor::from_system(SystemCursor::Crosshair).unwrap();
    cursor.set();

    let level = load_level("resources/levels/".to_string() + name + ".json");
    
    let mut tilemap = map::Map::new();
    tilemap.load(level.tilemap);

    let player: Entity = Entity::init(level.player);
    let mut goal = level.player;
    let mut trail = vec![];
    
    // load all sprites
    let mut img_floor = texture_creator.load_texture("resources/images/floor.png").unwrap();
    img_floor.set_blend_mode(BlendMode::Blend);
    let mut img_highlight = texture_creator.load_texture("resources/images/highlight.png").unwrap();
    img_highlight.set_blend_mode(BlendMode::Blend);
    img_highlight.set_alpha_mod(128);
    let mut img_cat = texture_creator.load_texture("resources/images/cat_idle_1.png").unwrap();
    img_cat.set_blend_mode(BlendMode::Blend);

    let (mut scale, (mut translation_x, mut translation_y)) = tilemap.get_scale_and_translation(canvas);
    println!("scale={} tx={} ty={}", scale, translation_x, translation_y);

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { return GameResult::Quit; },
                Event::Window { win_event: WindowEvent::Resized(_w, _h), ..} => {
                    (scale, (translation_x, translation_y)) = tilemap.get_scale_and_translation(canvas);
                    scale = scale.max(1);
                    println!("scale={} tx={} ty={}", scale, translation_x, translation_y);
                },
                _ => {}
            }
        }

        // get mouse position and determine selected tile
        let (mouse_x, mouse_y) = (
            event_pump.mouse_state().x(), 
            event_pump.mouse_state().y()
        );
        let (row, col) = tilemap.get_tile_index(
            (mouse_x - translation_x) / scale as i32, 
            (mouse_y - translation_y) / scale as i32
        );

        // if new tile selected, recalculate path
        if goal != (row, col) {
            goal = (row, col);
            trail = player.find_shortest_path(goal, &tilemap.tiles);
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // draw floor tiles
        for row in 0..tilemap.tiles.len() {
            for col in 0..tilemap.tiles[row].len() {
                let (x, y) = tilemap.get_tile_pos(row, col);
                if tilemap.tiles[row][col] == TileType::Floor {
                    canvas.copy(
                        &img_floor, 
                        None,
                        Rect::new(
                            x * scale as i32 + translation_x, 
                            y * scale as i32 + translation_y, 
                            28 * (scale as u32), 
                            19 * (scale as u32),
                        )
                    ).unwrap();
                }
            }
        }

        // draw highlights
        for (row, col) in trail.iter() {
            let (x, y) = tilemap.get_tile_pos(*row, *col);
            canvas.copy(
                &img_highlight, 
                None,
                Rect::new(
                    x * scale as i32 + translation_x, 
                    y * scale as i32 + translation_y, 
                    28 * (scale as u32), 
                    15 * (scale as u32),
                )
            ).unwrap();
        }

        // draw entities
        {
            // draw cat
            let (row, col) = player.get_position();
            let (x, y) = tilemap.get_tile_pos(row as usize, col as usize);
            canvas.copy(
                &img_cat, 
                None,
                Rect::new(
                    (x + 6) * scale as i32 + translation_x, 
                    (y - 6) * scale as i32 + translation_y, 
                    16 * (scale as u32), 
                    16 * (scale as u32),
                )
            ).unwrap();
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // return GameResult::Victory;
} 
