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
mod splash;

use level::GameResult;
use menu::MenuAction;

const DEBUG: bool = true;

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

    let levels = ["final_sewer", "final_sewer"];

    splash::show_splash(&mut canvas, &texture_creator, &mut event_pump, &mut font, "PIPALUK".to_string(), 0.75, 1500);

    canvas.present();
    let mut menu_result = menu::show_menu(&mut canvas, &mut texture_creator, &mut event_pump, &mut font, &mut music_mixer);
        if menu_result == MenuAction::Quit {
            return Ok(());
        }
    'running: loop {
        menu_result = MenuAction::NewGame;
        if menu_result == MenuAction::NewGame {
            let mut i: usize = 0;
            let n: usize = levels.len();
            while i < n{
                let game_result = level::play_level(&mut canvas, &mut texture_creator, &mut event_pump, &mut font, &mut music_mixer, levels[i]);
                match game_result {
                    GameResult::Quit => {
                        break 'running;
                    },
                    GameResult::Menu => {
                        menu_result = menu::show_menu(&mut canvas, &mut texture_creator, &mut event_pump, &mut font, &mut music_mixer);
                        match menu_result{
                            MenuAction::Quit => { break 'running },
                            MenuAction::NewGame => {},
                            MenuAction::Continue => {},
                            MenuAction::_Credits => {},
                            MenuAction::_Options => {}
                        }     
                    },
                    GameResult::Defeat => {
                        splash::show_splash(&mut canvas, &texture_creator, &mut event_pump, &mut font, "You Died".to_string(), 0.75, 4500);
                    },
                    GameResult::Victory => {
                        i += 1;
                        if i == n{
                            splash::show_splash(&mut canvas, &texture_creator, &mut event_pump, &mut font, "Congratulations you won!".to_string(), 0.75, 4500);
                            menu_result = menu::show_menu(&mut canvas, &mut texture_creator, &mut event_pump, &mut font, &mut music_mixer);
                            match menu_result{
                                MenuAction::Quit => { break 'running },
                                MenuAction::NewGame => {},
                                MenuAction::Continue => {},
                                MenuAction::_Credits => {},
                                MenuAction::_Options => {}
                            }
                        }
                        else {
                            splash::show_splash(&mut canvas, &texture_creator, &mut event_pump, &mut font, "You escaped!".to_string(), 0.75, 2500);
                        }
                    }
                }
            }
            break 'running; 
        }
    }
    Ok(())
}
