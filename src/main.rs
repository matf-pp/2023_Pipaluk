use level::GameResult;
use menu::MenuAction;

extern crate sdl2;

mod level;
mod menu;
mod map;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

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

    canvas.present();
    'running: loop {
        
        let menu_result = menu::show_menu(&mut canvas, &mut texture_creator, &mut event_pump,  &mut font);
        if menu_result == MenuAction::Quit {
            break 'running;
        }
        let menu_result = MenuAction::NewGame;
        if menu_result == MenuAction::NewGame {
            let game_result = level::play_level(&mut canvas, &mut texture_creator, &mut event_pump, "3");
            if game_result == GameResult::Quit {
                break 'running;
            }
        }
    }

    Ok(())
}
