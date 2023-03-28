use sdl2::event::Event;

extern crate sdl2;

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

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running }
                _ => {}
            }
        }
    }

    Ok(())
}
