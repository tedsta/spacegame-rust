use rsfml::window::{keyboard, mouse, event};
use rsfml::graphics::RenderWindow;

use module::MODULE_CATEGORIES;
use render::{Renderer, LASER_TEXTURE};

pub struct SpaceGui;

impl SpaceGui {
    pub fn new() -> SpaceGui {
        SpaceGui
    }
    
    pub fn update(&mut self, window: &mut RenderWindow) {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    keyboard::Escape => {},
                    _ => {},
                },
                event::KeyReleased{..} => {},
                event::MouseButtonPressed{button, x, y} => {
                    match button {
                        mouse::MouseLeft => self.on_mouse_left_pressed(x, y),
                        _ => {},
                    }
                }
                event::MouseButtonReleased{button, x, y} => {
                }
                event::NoEvent => break,
                _ => {}
            };
        }
    }
    
    pub fn draw(&self, renderer: &mut Renderer) {
        for category in MODULE_CATEGORIES.iter() {
            renderer.draw_texture(LASER_TEXTURE, 10.0 + (64.0*(category.id as u8 as f32)), 600.0);
        }
    }
    
    pub fn on_mouse_left_pressed(&mut self, x: i32, y: i32) {
        for category in MODULE_CATEGORIES.iter() {
            let icon_x = 10 + (64*(category.id as i32));
            let icon_y = 600i32;
            let icon_w = 64;
            let icon_h = 64;
            
            if x >= icon_x && x <= icon_x+icon_w && y >= icon_y && y <= icon_y+icon_h {
                println!("clicked!!");
            }
        }
    }
}