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

    for row in level.tilemap {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    // println!("{}", level.tilemap[0][0]);

    // let level_name = &level["name"];
    // let tilemap = &level["tilemap"];

    let scale = 4;

    let mut img_floor = texture_creator.load_texture("resources/images/floor.png").unwrap();
    img_floor.set_blend_mode(BlendMode::Blend);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return GameResult::Quit }
                _ => {}
            }
        }
        canvas.clear();

        // canvas.copy(&img_floor, None, Rect::new(0, 0, 28*scale, 19*scale)).unwrap();
        canvas.present();
    }

    return GameResult::Victory;
} 
