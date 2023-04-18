use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator, Texture};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, SystemCursor, MouseButton};
use std::collections::HashMap;

use crate::loader;
use crate::animation::Animation;
use crate::map::{Map, TileType};
use crate::entity::{Entity, Search};
use crate::player::Player;
use crate::robots::citizen::*;
// use crate::robots::policeman::*;
// use crate::robots::commando::*;

extern crate sdl2;

#[derive(PartialEq)]
pub enum GameResult {
    Quit,
    _Victory,
    _Defeat
}

pub struct State {
    pub tilemap: Map,
    pub player: Player,
    pub citizens: Vec<Citizen>,
    pub animation: Option<Animation>,
    pub trail: Vec<(usize, usize)>,
    pub goal: (usize, usize)
}

impl State {
    pub fn init(level: loader::LevelFile) -> Self {
        let mut tilemap = Map::new();
        tilemap.load(level.tilemap);
        let player: Player = Player::init(level.player);
        let citizens: Vec<Citizen> = level.citizens.iter().map(|&pos| Citizen::init(pos)).collect();
        Self {
            tilemap: tilemap,
            player: player,
            citizens: citizens,
            animation: None,
            trail: vec![],
            goal: (0, 0)
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
    let level = loader::load_level("resources/levels/".to_string() + name + ".json");
    let mut state: State = State::init(level);
    state.goal = state.player.get_position();
    state.trail = vec![];

    state.tilemap.calc_scale_and_translation(canvas);
    
    // load all sprites
    let mut sprites: HashMap<String, Texture> = HashMap::new();
    let texture_names = vec!["floor", "highlight", "citizen", "policeman", "cat_idle_1", "cat_run_0", "cat_run_1", "cat_run_2", "cat_run_3", "cat_run_4"];
    for name in texture_names.iter() { 
        sprites.insert(name.to_string(), texture_creator.load_texture(format!("resources/images/{}.png", *name)).unwrap()); 
    }
    sprites.get_mut("highlight").unwrap().set_alpha_mod(128);


    loop {

        // get mouse position and determine selected tile
        let (mouse_x, mouse_y) = (
            event_pump.mouse_state().x(), 
            event_pump.mouse_state().y()
        );
        let (row, col) = state.tilemap.get_tile_index(
            (mouse_x - state.tilemap.translation_x) / state.tilemap.scale as i32, 
            (mouse_y - state.tilemap.translation_y) / state.tilemap.scale as i32
        );

        // if new tile selected (and no animation is underway), recalculate path
        if state.goal != (row, col) && state.animation.is_none() {
            state.goal = (row, col); 
            state.trail = state.player.find_shortest_path(state.goal, &state.tilemap.tiles);
        }

        // ################################################## EVENT HANDLING ##
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { return GameResult::Quit; },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, ..} => {
                    if state.animation.is_none() {
                        let mut points = vec![state.player.pos];
                        points.append(&mut state.trail);
                        state.animation = Some(Animation::init(
                            points.iter().map(|(row, col)| state.tilemap.get_tile_pos(*row, *col)).collect(), 
                            vec!["cat_run_0", "cat_run_1", "cat_run_2", "cat_run_3", "cat_run_4"].iter().map(|name| name.to_string()).collect(),
                            3
                        ));
                    }
                },
                Event::Window { win_event: WindowEvent::Resized(_w, _h), ..} => {
                    let (scale, (translation_x, translation_y)) = state.tilemap.calc_scale_and_translation(canvas);
                    println!("Window resized : scale={} tx={} ty={}", scale, translation_x, translation_y);
                },
                _ => {}
            }
        }


        // ###################################################### GAME LOGIC ##

        if state.animation.is_none() {
            // player move
            // ...

            // citizens move
            for citizen in state.citizens.iter() {
                citizen.turn(&state);
            }
        }


        // ####################################################### RENDERING ##

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        render(canvas, &mut sprites, &mut state);
        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // return GameResult::Victory;
} 

struct Drawable {
    texture: String,
    x: i32,
    y: i32,
    flipped: bool,
    key: (usize, usize)
}

impl Drawable {
    pub fn init(texture: String, x: i32, y: i32, flipped: bool, key: (usize, usize)) -> Self {
        Self { texture, x, y, flipped, key }
    }
}

fn render(canvas: &mut WindowCanvas, sprites: &mut HashMap<String, Texture>, state: &mut State) {

    let mut drawables: Vec<Drawable> = vec![];

    // add floor tiles
    for row in 0..state.tilemap.tiles.len() {
        for col in 0..state.tilemap.tiles[row].len() {
            let (x, y) = state.tilemap.get_tile_pos(row, col);
            match state.tilemap.tiles[row][col] {
                TileType::Floor => { drawables.push(Drawable::init("floor".to_string(), x, y, false, (row, col))); },
                TileType::Wall => {},
                TileType::None => {}
            }
        }
    }

    // add highlights
    for (row, col) in state.trail.iter() {
        let (x, y) = state.tilemap.get_tile_pos(*row, *col);
        drawables.push(Drawable::init("highlight".to_string(), x, y, false, (*row, *col)));
    }

    // add cat
    {
        let (x, y, sprite, finished, flipped);
        if state.animation.is_none() {
            let (row, col) = state.player.get_position();
            (x, y) = state.tilemap.get_tile_pos(row as usize, col as usize);
        }
        else {
            ((x, y), sprite, finished, flipped) = state.animation.as_mut().unwrap().next_frame();
            state.player.pos = state.tilemap.get_tile_index(x+14, y+9);
            state.trail = state.player.find_shortest_path(state.goal, &state.tilemap.tiles);
            state.player.flipped = flipped.unwrap_or(state.player.flipped);
            state.player.current_sprite = sprite.to_string();
            if finished {
                println!("Animation finished");
                state.player.current_sprite = "cat_idle_1".to_string();
                state.animation = None;
            }
        }
        drawables.push(Drawable::init(state.player.current_sprite.clone(), x+6, y-6, state.player.flipped, state.player.get_position()));
    }

    // add citizens
    for citizen in state.citizens.iter() {
        let (row, col) = citizen.get_position();
        let (x, y) = state.tilemap.get_tile_pos(row as usize, col as usize);
        drawables.push(Drawable::init("citizen".to_string(), x+6, y-6, false, (row, col)));
    }

    drawables.sort_by_key(|d| d.key);

    for drawable in drawables.iter() {
        let tex = sprites.get_mut(drawable.texture.as_str()).unwrap();
        canvas.copy_ex(
            tex, 
            None,
            Rect::new(
                drawable.x * state.tilemap.scale as i32 + state.tilemap.translation_x, 
                drawable.y * state.tilemap.scale as i32 + state.tilemap.translation_y, 
                tex.query().width * (state.tilemap.scale as u32), 
                tex.query().height * (state.tilemap.scale as u32)
            ),
            0.0, 
            None, 
            drawable.flipped, 
            false
        ).unwrap();
    }

} 
