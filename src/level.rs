use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator, Texture};
use sdl2::image::LoadTexture;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::{Cursor, SystemCursor, MouseButton};
use std::collections::HashMap;

use crate::mixer::Mixer;
use crate::splash::{show_splash, SplashResult};
use crate::{loader, DEBUG};
use crate::animation::Animation;
use crate::map::{Map, TileType};
use crate::entity::{Entity, Search, Sight};
use crate::player::Player;
use crate::robots::citizen::*;
use crate::robots::policeman::*;
use crate::robots::commando::*;

extern crate sdl2;

const FRAME_DURATION: u64 = 50;

#[derive(PartialEq)]
pub enum GameResult {
    Quit,
    Menu,
    Victory,
    Defeat
}

#[derive(Clone)]
pub struct State {
    pub tilemap: Map,
    pub player: Player,
    pub citizens: Vec<Citizen>,
    pub policemen: Vec<Policeman>,
    pub commandos: Vec<Commando>,
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
        let commandos: Vec<Commando> = level.commandos
            .iter()
            .map(|&c| Commando::init(c.0, c.1))
            .collect();
        Self {
            tilemap: tilemap,
            player: player,
            citizens: citizens,
            policemen: policemen,
            commandos: commandos,
            animation: None,
            trail: vec![],
            goal: (0, 0)
        }
    }

    pub fn tile_free(&self, tile: (usize, usize)) -> bool {
        for citizen in self.citizens.iter() {
            if citizen.get_position() == tile { return false }
        }
        for policeman in self.policemen.iter() {
            if policeman.get_position() == tile { return false }
        }
        self.tilemap.passable(tile)
    }
}

pub fn play_level(
    canvas: &mut WindowCanvas, 
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &mut EventPump,
    font: &mut Font,
    music_mixer: &mut Mixer,
    name: &str
) -> GameResult {

    match show_splash(canvas, &texture_creator, event_pump, font, "Sewers".to_string(), 0.75, 3000) {
        SplashResult::Ok => {},
        SplashResult::Quit => { return GameResult::Quit; }
    }

    let cursor = Cursor::from_system(SystemCursor::Crosshair).unwrap();
    cursor.set();

    music_mixer.play_song("slow");

    // load level data and initialize game state
    let level = loader::load_level("resources/levels/".to_string() + name + ".json");
    let level_name = level.name.clone();
    let mut state: State = State::init(level);
    state.goal = state.player.get_position();
    state.trail = vec![];

    if DEBUG {
        state.tilemap.print();
    }

    match DEBUG {
        false => { state.tilemap.calc_scale_translation(canvas, state.player.get_position()); },
        true => { state.tilemap.calc_scale_translation_debug(canvas); }
    }
    
    // load all sprites
    let mut sprites: HashMap<String, Texture> = HashMap::new();
    let common_textures = vec![
        "highlight", "citizen_calm", "citizen_alert", "police_calm", "police_alert", "commando_calm", "commando_alert", 
        "cat_idle_1", "cat_run_0", "cat_run_1", "cat_run_2", "cat_run_3", "cat_run_4"
    ];
    for name in common_textures.iter() { 
        sprites.insert(name.to_string(), texture_creator.load_texture(format!("resources/images/{}.png", *name)).unwrap()); 
    }
    let level_textures = vec![
        "floor", "liquid", "wall_left", "wall_right", "wall_left_transparent", "wall_right_transparent",
        "border_left", "border_right", "border_corner"
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
            state.trail = state.player.find_shortest_path(state.goal, &state);
        }

        // player move and/or handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return GameResult::Quit; },
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => { return GameResult::Menu; },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, ..} => {
                    let turn_res = play_turn(canvas, &mut sprites, &mut state);
                    match turn_res {
                        TurnResult::Caught => { return GameResult::Defeat },
                        TurnResult::OK => {
                            // if state.player.pos == state.goal {return GameResult::_Victory;}
                            // else {}
                        },
                    }
                },
                // shortcuts to win/lose the game instantly in debug mode
                Event::KeyDown {keycode: Some(Keycode::W), ..} => { if DEBUG { return GameResult::Victory } }, 
                Event::KeyDown {keycode: Some(Keycode::L), ..} => { if DEBUG { return GameResult::Defeat } },
                Event::Window { win_event: WindowEvent::Resized(..), ..} => {
                    match DEBUG {
                        false => { state.tilemap.calc_scale_translation(canvas, state.player.get_position()); },
                        true => { state.tilemap.calc_scale_translation_debug(canvas); }
                    }
                },
                _ => {}
            }
        }

        render(canvas, &mut sprites, &mut state);
        std::thread::sleep(std::time::Duration::from_millis(FRAME_DURATION));
    }
} 

#[derive(PartialEq)]
pub enum TurnResult {
    Caught,
    OK
}

fn play_turn(canvas: &mut WindowCanvas, sprites: &mut HashMap<String, Texture>, state: &mut State) -> TurnResult {
    let mut turn_res = TurnResult::OK;

    // player turn
    let mut points = vec![state.player.pos];
    points.append(&mut state.trail);
    if points.len() != 1 {
        state.animation = Some(Animation::init(
        points.iter().map(|(row, col)| state.tilemap.get_tile_pos(*row, *col)).collect(), 
        vec!["cat_run_0", "cat_run_1", "cat_run_2", "cat_run_3", "cat_run_4"].iter().map(|name| name.to_string()).collect(),
        3
        ));
        while state.animation.is_some() {
            match DEBUG {
                false => { state.tilemap.calc_translation(canvas, state.player.get_position()); },
                true => { state.tilemap.calc_translation_debug(canvas); }
            }
            render(canvas, sprites, state);
            std::thread::sleep(std::time::Duration::from_millis(FRAME_DURATION));
        }
    }

    // citizens turn
    println!("Citizens turn...");
    for i in 0..state.citizens.len() {
        let state_copy = state.clone();
        for tile in state.citizens[i].turn(&state_copy).iter() {
            state.citizens[i].set_position(*tile);
            render(canvas, sprites, state);
            std::thread::sleep(std::time::Duration::from_millis(FRAME_DURATION));
        }
    }
    
    // policemen turn
    println!("Policemen turn...");
    for i in 0..state.policemen.len() {
        let state_copy = state.clone();
        for tile in state.policemen[i].turn(&state_copy).iter() {
            state.policemen[i].set_position(*tile);
            render(canvas, sprites, state);
            std::thread::sleep(std::time::Duration::from_millis(FRAME_DURATION));
        }
        
        if state.policemen[i].get_position() == state.player.pos {turn_res = TurnResult::Caught;}
    }
    
    // commandos turn
    println!("Commandos turn...");
    for i in 0..state.commandos.len() {
        let state_copy = state.clone();
        for tile in state.commandos[i].turn(&state_copy).iter() {
            state.commandos[i].set_position(*tile);
            render(canvas, sprites, state);
            std::thread::sleep(std::time::Duration::from_millis(FRAME_DURATION));
        }
        
        if state.commandos[i].get_position() == state.player.pos {turn_res = TurnResult::Caught;}
    }
    
    turn_res
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

    state.tilemap.calc_view();

    let mut drawables: Vec<Drawable> = vec![];

    // add floor tiles & walls
    for row in 0..state.tilemap.tiles.len() {
        for col in 0..state.tilemap.tiles[row].len() {
            let (x, y) = state.tilemap.get_tile_pos(row, col);
            let (prow, pcol) = state.player.get_position();
            let (drow, dcol) = ((row as i32 - prow as i32).clamp(-1, 1), (col as i32 - pcol as i32).clamp(-1, 1));
            match state.tilemap.tiles[row][col] {
                TileType::Floor => { drawables.push(Drawable::init("floor".to_string(), x, y, false, (row, col))); },
                TileType::Wall => {
                    match state.tilemap.tiles[row-1][col-1] {
                        TileType::Wall | TileType::None => {},
                        _ => {
                            if drow == 1 && dcol == 1 {
                                if state.tilemap.tiles[row-1][col] == TileType::Wall && state.tilemap.tiles[row][col-1] == TileType::Wall {
                                    drawables.push(Drawable::init("border_corner".to_string(), x+12, y+1, false, (row, col)));
                                }
                            }
                        }
                    }
                    match state.tilemap.tiles[row-1][col] {
                        TileType::Wall | TileType::None => {},
                        _ => {
                            if drow == 1 {
                                drawables.push(Drawable::init("border_left".to_string(), x+12, y+1, false, (row, col)));
                            }
                        }
                    }
                    match state.tilemap.tiles[row][col-1] {
                        TileType::Wall | TileType::None => {},
                        _ => {
                            if dcol == 1 {
                                drawables.push(Drawable::init("border_right".to_string(), x, y+1, false, (row, col)));
                            }
                        }
                    }
                    match state.tilemap.tiles[row+1][col] {
                        TileType::Wall | TileType::None => {},
                        _ => {
                            if drow == -1 {
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
                    }
                    match state.tilemap.tiles[row][col+1] {
                        TileType::Wall | TileType::None => {},
                        _ => {
                            if dcol == -1 {
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
            state.trail = state.player.find_shortest_path(state.goal, &state);
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
        match citizen.mode {
            CitizenState::CALM => {
                drawables.push(Drawable::init("citizen_calm".to_string(), x+6, y-6, false, (row, col)));
            },
            CitizenState::PANIC => {
                drawables.push(Drawable::init("citizen_alert".to_string(), x+6, y-6, false, (row, col)));
            }
        }
    }
    
    // add policemen
    for policeman in state.policemen.iter() {
        let (row, col) = policeman.get_position();
        let (x, y) = state.tilemap.get_tile_pos(row as usize, col as usize);
        match policeman.sees(state.player.get_position(), &state.tilemap.tiles) {
            true => {
                drawables.push(Drawable::init("police_alert".to_string(), x+6, y-6, false, (row, col)));

            },
            false => {
                drawables.push(Drawable::init("police_calm".to_string(), x+6, y-6, false, (row, col)));
            }
        }
    }
    
    // add commandos
    for commando in state.commandos.iter() {
        let (row, col) = commando.get_position();
        let (x, y) = state.tilemap.get_tile_pos(row as usize, col as usize);
        match commando.sees(state.player.get_position(), &state.tilemap.tiles) {
            true => {
                drawables.push(Drawable::init("commando_alert".to_string(), x+6, y-6, false, (row, col)));
            },
            false => {
                drawables.push(Drawable::init("commando_calm".to_string(), x+6, y-6, false, (row, col)));
            }
        }
    }

    // sort and draw everything
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    drawables.sort_by_key(|d| d.key);
    for drawable in drawables.iter() {
        let tex = sprites.get_mut(drawable.texture.as_str()).unwrap();
        let (row, col) = drawable.key;
        if !state.player.sees((row, col), &state.tilemap.tiles) {
            if DEBUG { tex.set_color_mod(128, 128, 128); }
            else { continue; }
        }
        else if !DEBUG {
            let distance = state.player.distance_to((row, col));
            let color = 256.0 * (1.0 - (distance / Player::DISTANCE as f32).powf(2.0)).max(0.0);
            let color = color as u8;
            tex.set_color_mod(color, color, color); 
        }
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
        tex.set_color_mod(255, 255, 255);
    }
    canvas.present();
} 
