extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;

use crate::mixer::Mixer;

#[derive(PartialEq, Clone, Copy)]
pub enum MenuAction {
    Quit,
    NewGame,
    Continue,
    _Credits,
    _Options
}

struct MenuButton {
    text: String,
    action: MenuAction,
    enabled: bool,
    rect: Rect,
    hovered: u8
}

impl MenuButton {
    
    pub fn new<'a>(
        text: String, 
        action: MenuAction,
        enabled: bool,
        rect: Rect,
    ) -> Self {

        Self {
            text: text,
            action: action,
            enabled: enabled,
            rect: rect,
            hovered: 5
        }
    }

    pub fn rect(&self, scale: f32) -> Rect {
        Self::scale_rect(self.rect, scale)
    }

    pub fn scale_rect(rect: Rect, scale: f32) -> Rect {
        Rect::new(
            (rect.x() as f32 * scale) as i32,
            (rect.y() as f32 * scale) as i32,
            (rect.width() as f32 * scale) as u32,
            (rect.height() as f32 * scale) as u32
        )
    }

    pub fn render(
        &mut self, 
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        font: &mut Font,
        global_scale: f32
    ) {

        let mut background_surface = Surface::new(
            self.rect.width(), 
            self.rect.height(), 
            sdl2::pixels::PixelFormatEnum::RGBA8888,
        )
        .unwrap();

        background_surface.fill_rect(
            Rect::new(0, 0, self.rect.width(), self.rect.height()),
            Color::RGBA(30+self.hovered, 30+self.hovered, 30+self.hovered, 128)
        ).unwrap();

        let background_texture = texture_creator
            .create_texture_from_surface(&background_surface).map_err(|e| e.to_string()).unwrap();

        canvas.copy(
            &background_texture,
            None,
            self.rect(global_scale)
        )
        .unwrap();

        let mut text_color = Color::RGBA(255, 255, 255, 255);
        if !self.enabled {
            text_color = Color::RGBA(128, 128, 128, 255);
        }
        let text_surface = font
            .render(&self.text)
            .blended(text_color)
            .map_err(|e| e.to_string())
            .unwrap();

        let scale = (text_surface.rect().width() as f32) / (text_surface.rect().height() as f32);

        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface).map_err(|e| e.to_string()).unwrap(); 
        
        canvas.copy(
            &text_texture,
            None, 
            Self::scale_rect(Rect::new( 
                self.rect.x()+20, 
                self.rect.y()+10, 
                (((self.rect.height()-10) as f32)*scale) as u32, 
                self.rect.height()-10
            ), global_scale)
        ).unwrap();

    }
}

pub fn show_menu(
    canvas: &mut WindowCanvas, 
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &mut EventPump,
    font: &mut Font,
    music_mixer: &mut Mixer
) -> MenuAction {
    println!("SHOW MENU");
    music_mixer.play_song("slow");

    let background = texture_creator.load_texture("resources/images/menu_background.png").unwrap();
    let foreground = texture_creator.load_texture("resources/images/menu_foreground.png").unwrap();

    let cat = vec![
        texture_creator.load_texture("resources/images/cat_idle_1.png").unwrap(),
        texture_creator.load_texture("resources/images/cat_idle_2.png").unwrap()
    ];

    let mut buttons = vec![
        MenuButton::new(
            "New Game".to_string(), 
            MenuAction::NewGame, 
            true,
            Rect::new(0, 20, 340, 60)
        ),
        MenuButton::new(
            "Continue".to_string(), 
            MenuAction::Continue, 
            false,
            Rect::new(0, 100, 280, 60)
        ),
        MenuButton::new(
            "Quit".to_string(), 
            MenuAction::Quit, 
            true,
            Rect::new(0, 180, 220, 60)
        )
    ];

    let mut counter = 0;

    loop {

        let (canvas_x, canvas_y) = canvas.output_size().unwrap();
        let scale_max = (canvas_x as f32 / 800 as f32).max(canvas_y as f32 / 600 as f32);
        let scale_min = (canvas_x as f32 / 800 as f32).min(canvas_y as f32 / 600 as f32);
        let scale = (scale_min+scale_max)/2.0;
        let translation_x = (canvas_x as f32 - 800.0 * scale_max) as i32 / 2;
        let translation_y = (canvas_y as f32 - 600.0 * scale_max) as i32 / 2;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { return MenuAction::Quit },
                Event::MouseButtonDown { x, y, .. } => {
                    for i in 0..3 {
                        if buttons[i].enabled && buttons[i].rect(scale).contains_point(Point::new(x, y)) {
                            println!("\"{}\" pressed", buttons[i].text);
                            return buttons[i].action;
                        }
                    }
                }
                _ => {}
            }
        }

        let (x, y) = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
        for i in 0..3 {
            if buttons[i].rect(scale).contains_point(Point::new(x, y)) {
                buttons[i].hovered = (buttons[i].hovered + 5).min(50);
            }
            else {
                buttons[i].hovered = (buttons[i].hovered - 5).max(5);
            }
        }

        canvas.clear();
        
        canvas.copy(
            &background, 
            None, 
            Rect::new(
                translation_x,
                translation_y,
                (800.0 * scale_max) as u32,
                (600.0 * scale_max) as u32
            )
        ).unwrap();

        canvas.copy_ex(
            &cat[counter/20], 
            None, 
            Rect::new(
                translation_x + (410.0 * scale_max) as i32, 
                translation_y + (90.0 * scale_max) as i32, 
                (400.0 * scale_max) as u32, 
                (400.0 * scale_max) as u32
            ), 
            0.0, 
            None, 
            true, 
            false
        ).unwrap();

        canvas.copy(
            &foreground, 
            None, 
            Rect::new(
                translation_x,
                translation_y,
                (800.0 * scale_max) as u32,
                (600.0 * scale_max) as u32
            )
        ).unwrap();

        for i in 0..3 {
            buttons[i].render(canvas, texture_creator, font, scale);
        }

        counter = (counter + 1) % 40;
        
        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}