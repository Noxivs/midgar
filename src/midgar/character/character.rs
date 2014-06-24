use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape};

use game_state::Viewable;
use resource_loader::ResourceLoader;

pub struct Character<'s> {
    texture: RectangleShape<'s>
}

impl<'s> Character<'s> {
    pub fn new(resource_loader: &'s ResourceLoader) -> Character<'s> {
        let mut texture = RectangleShape::new().unwrap();
        texture.set_size(&Vector2f::new(512. , 512. ));
        texture.set_texture(resource_loader.get_texture(1), false);
        Character { 
            texture: texture
        }
    }
}

impl<'s> Viewable for Character<'s> {
    fn update(&self, render_window: &mut RenderWindow) -> Option<i32> {
        None
    }

    fn draw(&self, render_window: &mut RenderWindow) -> () {
        render_window.draw(&self.texture);
    }
}