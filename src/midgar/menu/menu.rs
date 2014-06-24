use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape};
use rsfml::window::keyboard;

use event_handler::EventHandler;
use game_state::Viewable;
use resource_loader::ResourceLoader;

pub struct Menu<'s> {
    welcome_rc: RectangleShape<'s>
}

impl<'s> Menu<'s> {
    pub fn new(resource_loader: &'s ResourceLoader) -> Menu<'s> {
        let mut welcome = RectangleShape::new().unwrap();
        welcome.set_size(&Vector2f::new(512. , 512. ));
        welcome.set_texture(resource_loader.get_texture(0), false);
        Menu {
            welcome_rc: welcome
        }
    }
}

impl<'s> Viewable for Menu<'s> {
    fn update(&self, render_window: &mut RenderWindow, event_handler: &EventHandler) -> Option<i32> {

        if event_handler.is_key_pressed(keyboard::Space) {
            return Some(1)
        }

        None
    }

    fn draw(&self, render_window: &mut RenderWindow) -> () {
        render_window.draw(&self.welcome_rc);
    }
}