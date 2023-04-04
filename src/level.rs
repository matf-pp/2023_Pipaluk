use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator, Texture, BlendMode };
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, SystemCursor};

const TILEMAP_WIDTH: i32 = 100;
const TILEMAP_HEIGHT: i32 = 100;
const TILE_OFFSET: i32 = 1;

extern crate sdl2;

#[derive(Serialize, Deserialize)]
struct Level {
    name: String,
    tilemap: Vec<Vec<u32>>,
}

fn load_level(path: String) -> Level {
    println!("loading {path}!");
    let source = std::fs::read_to_string(path).expect("Failed to read level file");
    let parsed_level: Level = serde_json::from_str(&source).expect("Failed to parse level file");
    return parsed_level;
}

#[derive(PartialEq)]
pub enum GameResult {
    Quit,
    Victory,
    Defeat
}

pub fn play_level(
    canvas: &mut WindowCanvas, 
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &mut EventPump,
    name: &str
) -> GameResult {
    let level = load_level("resources/levels/".to_string() + name + ".json");

    let cursor = Cursor::from_system(SystemCursor::Crosshair).unwrap();
    cursor.set();
    
    /*for row in &level.tilemap {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }*/
    //println!("{}", level.tilemap[0][0]);

    // let level_name = &level["name"];
    // let tilemap = &level["tilemap"];

    let scale = 3;

    let mut img_floor = texture_creator.load_texture("resources/images/floor.png").unwrap();
    img_floor.set_blend_mode(BlendMode::Blend);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { return GameResult::Quit },
                Event::MouseButtonDown {x, y, ..}
                | Event::MouseButtonUp {x, y, ..} => {println!("Mouse click position: ({}, {})", x, y)},
                Event::MouseMotion {x, y, ..} => {println!("Mouse position: ({}, {})", x, y)}
                _ => {}
            }
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // draw floor tiles
        for row in 0..level.tilemap.len() {
            for col in 0..level.tilemap[row].len() {
                if level.tilemap[row][col] == 1 {
                    canvas.copy(
                        &img_floor, 
                        None,
                        Rect::new(
                            400 - 14 - (row as i32)*14*scale + (col as i32)*14*scale, 
                            (row as i32)*7*scale + (col as i32)*7*scale, 
                            28 * (scale as u32), 
                            19 * (scale as u32),
                        )
                    ).unwrap();
                }
            }
        }
        canvas.present();
    }

    return GameResult::Victory;
} 
