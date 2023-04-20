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
use crate::robots::policeman::*;
//use crate::robots::commando::*;

extern crate sdl2;

#[derive(PartialEq)]
pub enum GameResult {
    Quit,
    _Victory,
    _Defeat
}

#[derive(Clone)]
pub struct State {
    pub tilemap: Map,
    pub player: Player,
    pub citizens: Vec<Citizen>,
    pub policemen: Vec<Policeman>,
    //pub commando: Vec<Commando>,
    pub animation: Option<Animation>,
    pub trail: Vec<(usize, usize)>,
    pub goal: (usize, usize)
}

impl State {
    pub fn init(level: loader::LevelFile) -> Self {
        let mut tilemap = Map::new();
        tilemap.load(level.tilemap);
        let player: Player = Player::init(level.player);
        let citizens: Vec<Citizen> = level.citizens
            .iter()
            .map(|&pos| Citizen::init(pos, CitizenState::CALM))
            .collect();
        let policemen: Vec<Policeman> = level.policemen
            .iter()
            .map(|&p| Policeman::init(p.0, p.1))
            .collect();
            
        Self {
            tilemap: tilemap,
            player: player,
            citizens: citizens,
            policemen: policemen,
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
    let level_name = level.name.clone();
    let mut state: State = State::init(level);
    state.goal = state.player.get_position();
    state.trail = vec![];

    state.tilemap.calc_scale_and_translation(canvas);
    state.tilemap._print();
    
    // load all sprites
    let mut sprites: HashMap<String, Texture> = HashMap::new();
    let common_textures = vec![
        "highlight", "citizen", "policeman", 
        "cat_idle_1", "cat_run_0", "cat_run_1", "cat_run_2", "cat_run_3", "cat_run_4"
    ];
    for name in common_textures.iter() { 
        sprites.insert(name.to_string(), texture_creator.load_texture(format!("resources/images/{}.png", *name)).unwrap()); 
    }
    let level_textures = vec![
        "floor", "liquid", "wall_left", "wall_right", "wall_left_transparent", "wall_right_transparent"
    ];
    for name in level_textures.iter() { 
        sprites.insert(
            name.to_string(), 
            texture_creator.load_texture(format!("resources/images/{}/{}.png", level_name, *name))
            .unwrap_or(texture_creator.load_texture(format!("resources/images/{}.png", *name)).unwrap())
        ); 
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

        // player move and/or handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { return GameResult::Quit; },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, ..} => {
                    play_turn(canvas, &mut sprites, &mut state);
                },
                Event::Window { win_event: WindowEvent::Resized(_w, _h), ..} => {
                    let (scale, (translation_x, translation_y)) = state.tilemap.calc_scale_and_translation(canvas);
                    println!("Window resized : scale={} tx={} ty={}", scale, translation_x, translation_y);
                },
                _ => {}
            }
        }

        render(canvas, &mut sprites, &mut state);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // return GameResult::Victory;
} 

fn play_turn(canvas: &mut WindowCanvas, sprites: &mut HashMap<String, Texture>, state: &mut State) {
    
    // player turn
    let mut points = vec![state.player.pos];
    points.append(&mut state.trail);
    if points.len() == 1 {
        return;
    }
    state.animation = Some(Animation::init(
        points.iter().map(|(row, col)| state.tilemap.get_tile_pos(*row, *col)).collect(), 
        vec!["cat_run_0", "cat_run_1", "cat_run_2", "cat_run_3", "cat_run_4"].iter().map(|name| name.to_string()).collect(),
        3
    ));
    while state.animation.is_some() {
        render(canvas, sprites, state);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // citizens turn
    println!("Citizens turn...");
    for i in 0..state.citizens.len() {
        let state_copy = state.clone();
        state.citizens[i].turn(&state_copy);
        render(canvas, sprites, state);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    // policemen turn
    println!("Policemen turn...");
    for i in 0..state.policemen.len() {
        let state_copy = state.clone();
        state.policemen[i].turn(&state_copy);
        render(canvas, sprites, state);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
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

    // add floor tiles & walls
    for row in 0..state.tilemap.tiles.len() {
        for col in 0..state.tilemap.tiles[row].len() {
            let (x, y) = state.tilemap.get_tile_pos(row, col);
            match state.tilemap.tiles[row][col] {
                TileType::Floor => { drawables.push(Drawable::init("floor".to_string(), x, y, false, (row, col))); },
                TileType::Wall => {
                    match state.tilemap.tiles[row+1][col] {
                        TileType::Wall | TileType::None => {},
                        _ => {
                            match state.tilemap.tiles[row][col-1] {
                                TileType::Wall | TileType::None => {
                                    drawables.push(Drawable::init("wall_right".to_string(), x, y-9, false, (row, col)));
                                },
                                _ => {
                                    drawables.push(Drawable::init("wall_right_transparent".to_string(), x, y-9, false, (row, col)));
                                }
                            }
                        }
                    }
                    match state.tilemap.tiles[row][col+1] {
                        TileType::Wall | TileType::None => {},
                        _ => {
                            match state.tilemap.tiles[row-1][col] {
                                TileType::Wall | TileType::None => {
                                    drawables.push(Drawable::init("wall_left".to_string(), x+12, y-9, false, (row, col)));
                                },
                                _ => {
                                    drawables.push(Drawable::init("wall_left_transparent".to_string(), x+12, y-9, false, (row, col)));
                                }
                            }
                        }
                    }
                },
                TileType::Liquid => { drawables.push(Drawable::init("liquid".to_string(), x, y+3, false, (row, col))); },
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
    
    // add policemen
    for policeman in state.policemen.iter() {
        let (row, col) = policeman.get_position();
        let (x, y) = state.tilemap.get_tile_pos(row as usize, col as usize);
        drawables.push(Drawable::init("policeman".to_string(), x+6, y-6, false, (row, col)));
    }

    // sort and draw everything
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
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
    canvas.present();
} 
