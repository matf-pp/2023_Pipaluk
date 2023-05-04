extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::ttf::Font;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator};
use sdl2::video::{WindowContext, FullscreenType};
use sdl2::rect::Rect;
use sdl2::event::Event;

pub enum SplashResult {
    Ok,
    Quit
}

pub fn show_splash(
    canvas: &mut WindowCanvas, 
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &mut EventPump,
    font: &mut Font,
    message: String,
    size: f32,
    duration: i32,
) -> SplashResult {

    let text_surface = font
            .render(&message)
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())
            .unwrap();

    let mut text_texture = texture_creator
    .create_texture_from_surface(&text_surface).map_err(|e| e.to_string()).unwrap(); 
    
    let mut passed: i32 = 0;

    loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return SplashResult::Quit },
                Event::KeyDown { keycode: Some(Keycode::F11), ..} => {
                    match canvas.window().fullscreen_state() {
                        FullscreenType::Off => { canvas.window_mut().set_fullscreen(FullscreenType::True).unwrap() },
                        _ => { canvas.window_mut().set_fullscreen(FullscreenType::Off).unwrap() }
                    }
                },
                Event::KeyDown {..}
                | Event::MouseButtonDown {..} => { return SplashResult::Ok },
                _ => {}
            }
        }

        if passed >= duration {
            return SplashResult::Ok;
        }

        let (canvas_x, canvas_y) = canvas.output_size().unwrap();
        let text_width = text_texture.query().width;
        let text_height = text_texture.query().height;
        let scale = canvas_x as f32 * size / text_width as f32; 

        canvas.clear();

        let mut alpha: u8 = 255;
        if passed*3 <= duration {
            alpha = (255 * passed * 3 / duration) as u8;
        }
        if passed*3/2 >= duration {
            alpha = (255 - 255 * (passed * 3 - duration * 2) / duration) as u8;
        }
        text_texture.set_alpha_mod(alpha);

        canvas.copy(
            &text_texture,
            None, 
            Rect::new( 
                (canvas_x as f32 / 2.0 - text_width as f32 * scale / 2.0) as i32,
                (canvas_y as f32 / 2.0 - text_height as f32 * scale / 2.0) as i32,
                (text_width as f32 * scale) as u32,
                (text_height as f32 * scale) as u32
            )
        ).unwrap();
        
        canvas.present();

        passed += 50;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}