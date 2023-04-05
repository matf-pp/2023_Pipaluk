use sdl2::surface::Surface;
use sdl2::ttf::Font;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::render::{ WindowCanvas, TextureCreator, Texture, BlendMode };
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::video::WindowContext;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate sdl2;

#[derive(PartialEq, Clone, Copy)]
pub enum MenuAction {
    Quit,
    NewGame,
    Continue,
    Credits,
    Options
}

struct MenuButton {
    text: String,
    action: MenuAction,
    enabled: bool,
    rect: Rect,
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
        }
    }

    pub fn render(
        &mut self, 
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        font: &mut Font,
    ) {

        let mut background_surface = Surface::new(
            self.rect.width(), 
            self.rect.height(), 
            sdl2::pixels::PixelFormatEnum::RGBA8888,
        )
        .unwrap();

        background_surface.fill_rect(
            Rect::new(0, 0, self.rect.width(), self.rect.height()),
            Color::RGBA(30, 30, 30, 128)
        ).unwrap();

        let background_texture = texture_creator
            .create_texture_from_surface(&background_surface).map_err(|e| e.to_string()).unwrap();

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
            &background_texture,
            None,
            self.rect
        )
        .unwrap();
        
        canvas.copy(
            &text_texture,
            None, 
            Rect::new( 
                self.rect.x()+20, 
                self.rect.y()+10, 
                (((self.rect.height()-10) as f32)*scale) as u32, 
                self.rect.height()-10
            )
        ).unwrap();

    }
}

pub fn show_menu(
    canvas: &mut WindowCanvas, 
    texture_creator: &TextureCreator<WindowContext>,
    event_pump: &mut EventPump,
    font: &mut Font,
) -> MenuAction {
    println!("SHOW MENU");

    let background = texture_creator.load_texture("resources/images/menu_background.png").unwrap();
    let foreground = texture_creator.load_texture("resources/images/menu_foreground.png").unwrap();

    let cat = vec![
        texture_creator.load_texture("resources/images/cat_idle_1.png").unwrap(),
        texture_creator.load_texture("resources/images/cat_idle_2.png").unwrap()
    ];
    

    let mut button1 = MenuButton::new(
        "New Game".to_string(), 
        MenuAction::NewGame, 
        true,
        Rect::new(0, 20, 340, 60)
    );

    let mut button2 = MenuButton::new(
        "Continue".to_string(), 
        MenuAction::Continue, 
        false,
        Rect::new(0, 100, 280, 60)
    );

    let mut button3 = MenuButton::new(
        "Quit".to_string(), 
        MenuAction::Quit, 
        true,
        Rect::new(0, 180, 220, 60)
    );

    let mut buttons = vec![button1, button2, button3];

    let mut counter = 0;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { return MenuAction::Quit }
                Event::MouseButtonDown { x, y, .. } => {
                    for i in 0..3 {
                        if buttons[i].enabled && buttons[i].rect.contains_point(Point::new(x, y)) {
                            println!("\"{}\" pressed", buttons[i].text);
                            return buttons[i].action;
                        }
                    }
                }
                _ => {}
            }
        }

        canvas.clear();

        canvas.copy(&background, None, None).unwrap();

        canvas.copy_ex(
            &cat[counter/20], 
            None, 
            Rect::new(420, 100, 400, 400), 
            0.0, 
            None, 
            true, 
            false
        ).unwrap();

        canvas.copy(&foreground, None, None).unwrap();

        for i in 0..3 {
            buttons[i].render(canvas, texture_creator, font);
        }

        counter = (counter + 1) % 40;
        
        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}