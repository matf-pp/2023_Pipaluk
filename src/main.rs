extern crate sdl2;

mod mixer;
mod level;
mod menu;
mod map;
mod entity;
mod player;
mod robots;
mod loader;
mod animation;

use level::GameResult;
use menu::MenuAction;

const DEBUG: bool = false;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _audio = sdl_context.audio()?;

    let window = video_subsystem.window(
        "Pipaluk",
        800,
        600
    )
    .position_centered()
    .resizable()
    .build()
    .unwrap();

    let mut canvas = window.into_canvas()
    .build()
    .unwrap();

    let mut texture_creator = canvas.texture_creator(); 

    let mut event_pump = sdl_context.event_pump()?;

    let ttf_context = sdl2::ttf::init()
        .map_err(|e| e.to_string())?;
    let mut font = ttf_context
        .load_font("resources/fonts/Minecraft.ttf", 64)?;

    let mut music_mixer = mixer::Mixer::init();

    canvas.present();
    'running: loop {
        
        let menu_result = menu::show_menu(&mut canvas, &mut texture_creator, &mut event_pump, &mut font, &mut music_mixer);
        if menu_result == MenuAction::Quit {
            break 'running;
        }
        let menu_result = MenuAction::NewGame;
        if menu_result == MenuAction::NewGame {
            let game_result = level::play_level(&mut canvas, &mut texture_creator, &mut event_pump, &mut music_mixer, "final_sewer");
            if game_result == GameResult::Quit {
                break 'running;
            }
        }
    }

    Ok(())
}
