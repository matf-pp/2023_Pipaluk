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

use crate::map::{Map, TileType};
use crate::entity::{Entity, Search};
use crate::player::Player;
use crate::robots::citizen::*;
// use crate::robots::policeman::*;
// use crate::robots::commando::*;

extern crate sdl2;

#[derive(Serialize, Deserialize)]
pub struct LevelFile {
    name: String,
    tilemap: Vec<Vec<u32>>,
    player: (usize, usize),
    citizens: Vec<(usize, usize)>
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

pub struct State {
    pub tilemap: Map,
    pub player: Player,
    pub citizens: Vec<Citizen>
}

impl State {
    pub fn init(level: LevelFile) -> Self {
        let mut tilemap = Map::new();
        tilemap.load(level.tilemap);
        let player: Player = Player::init(level.player);
        let citizens: Vec<Citizen> = level.citizens.iter().map(|&pos| Citizen::init(pos)).collect();
        Self {
            tilemap: tilemap,
            player: player,
            citizens: citizens
        }
    }
}

pub fn play_level(
    canvas: &mut WindowCanvas, 
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &mut EventPump,
    name: &str
) -> GameResult {
    let cursor = Cursor::from_system(SystemCursor::Crosshair).unwrap();
    cursor.set();

    // load level data and initialize game state
    let level = load_level("resources/levels/".to_string() + name + ".json");
    let mut state = State::init(level);
    let mut goal = state.player.get_position();
    let mut trail = vec![];
    
    // load all sprites
    let mut img_floor = texture_creator.load_texture("resources/images/floor.png").unwrap();
    img_floor.set_blend_mode(BlendMode::Blend);
    let mut img_highlight = texture_creator.load_texture("resources/images/highlight.png").unwrap();
    img_highlight.set_blend_mode(BlendMode::Blend);
    img_highlight.set_alpha_mod(128);
    let mut img_cat = texture_creator.load_texture("resources/images/cat_idle_1.png").unwrap();
    img_cat.set_blend_mode(BlendMode::Blend);
    let mut img_citizen = texture_creator.load_texture("resources/images/citizen.png").unwrap();
    img_citizen.set_blend_mode(BlendMode::Blend);

    let (mut scale, (mut translation_x, mut translation_y)) = state.tilemap.get_scale_and_translation(canvas);
    println!("scale={} tx={} ty={}", scale, translation_x, translation_y);

    loop {

        // ################################################## EVENT HANDLING ##
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { return GameResult::Quit; },
                Event::Window { win_event: WindowEvent::Resized(_w, _h), ..} => {
                    (scale, (translation_x, translation_y)) = state.tilemap.get_scale_and_translation(canvas);
                    scale = scale.max(1);
                    println!("scale={} tx={} ty={}", scale, translation_x, translation_y);
                },
                _ => {}
            }
        }


        // ###################################################### GAME LOGIC ##

        // get mouse position and determine selected tile
        let (mouse_x, mouse_y) = (
            event_pump.mouse_state().x(), 
            event_pump.mouse_state().y()
        );
        let (row, col) = state.tilemap.get_tile_index(
            (mouse_x - translation_x) / scale as i32, 
            (mouse_y - translation_y) / scale as i32
        );

        // if new tile selected, recalculate path
        if goal != (row, col) {
            goal = (row, col);
            trail = state.player.find_shortest_path(goal, &state.tilemap.tiles);
        }

        // player move
        // ...

        // citizens move
        for citizen in state.citizens.iter() {
            citizen.turn(&state);
        }


        // ####################################################### RENDERING ##

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // draw floor tiles
        for row in 0..state.tilemap.tiles.len() {
            for col in 0..state.tilemap.tiles[row].len() {
                let (x, y) = state.tilemap.get_tile_pos(row, col);
                if state.tilemap.tiles[row][col] == TileType::Floor {
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
            let (x, y) = state.tilemap.get_tile_pos(*row, *col);
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
            let (row, col) = state.player.get_position();
            let (x, y) = state.tilemap.get_tile_pos(row as usize, col as usize);
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

            // draw citizens
            for citizen in state.citizens.iter() {
                let (row, col) = citizen.get_position();
                let (x, y) = state.tilemap.get_tile_pos(row as usize, col as usize);
                canvas.copy(
                    &img_citizen, 
                    None,
                    Rect::new(
                        (x + 6) * scale as i32 + translation_x, 
                        (y - 6) * scale as i32 + translation_y, 
                        16 * (scale as u32), 
                        16 * (scale as u32),
                    )
                ).unwrap();
            }
        }

        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // return GameResult::Victory;
} 
