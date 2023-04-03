use level::GameResult;
use sdl2::{event::Event, render::{WindowCanvas, TextureCreator}, video::WindowContext};

extern crate sdl2;

mod level;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(
        "Pipaluk",
        800,
        600
    )
    .position_centered()
    .build()
    .unwrap();

    let mut canvas = window.into_canvas()
    .build()
    .unwrap();

    let mut texture_creator = canvas.texture_creator(); 

    let mut event_pump = sdl_context.event_pump()?;

    canvas.present();
    'running: loop {
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit {..} => { break 'running }
        //         _ => {}
        //     }
        // }

        let game_result = level::play_level(&mut canvas, &mut texture_creator, &mut event_pump, "1");

        if game_result == GameResult::Quit {
            break 'running;
        }
    }

    Ok(())
}
