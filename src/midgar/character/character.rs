use std::rc::Rc;
use std::cell::RefCell;

use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape, Texture};

use event_handler::EventHandler;
use game_state::Viewable;
use resource_loader::ResourceLoader;

pub struct Character<'s> {
    texture: RectangleShape<'s>,
    clerc_rc: RectangleShape<'s>,
    magician_rc: RectangleShape<'s>,
    ninja_rc: RectangleShape<'s>,
    warrior_rc: RectangleShape<'s>
}

impl<'s> Character<'s> {
    pub fn new(resource_loader: &'s ResourceLoader) -> Character<'s> {
        let mut texture = RectangleShape::new().unwrap();
        texture.set_size(&Vector2f::new(512. , 512. ));
        texture.set_texture(resource_loader.get_texture(1), false);

        Character { 
            texture: texture,
            clerc_rc: Character::create_rc(&Vector2f::new(96. , 156. ), &Vector2f::new(70. , 60. ), resource_loader.get_texture(2)),
            magician_rc: Character::create_rc(&Vector2f::new(92. , 159. ), &Vector2f::new(172. , 60. ), resource_loader.get_texture(3)),
            ninja_rc: Character::create_rc(&Vector2f::new(101. , 164. ), &Vector2f::new(274. , 60. ), resource_loader.get_texture(4)),
            warrior_rc: Character::create_rc(&Vector2f::new(88. , 147. ), &Vector2f::new(376. , 60. ), resource_loader.get_texture(5))
        }
    }

    pub fn create_rc<'s>(size: &Vector2f,  position: &Vector2f, texture : &'s Texture) -> RectangleShape<'s> {
        let mut rc = RectangleShape::new().unwrap();
        rc.set_size(size);
        rc.set_position(position);
        rc.set_texture(texture, false);
        rc
    }
}

impl<'s> Viewable for Character<'s> {
    fn update(&self, render_window: &mut RenderWindow, event_handler: &EventHandler) -> Option<i32> {
        None
    }

    fn draw(&self, render_window: &mut RenderWindow) -> () {
        render_window.draw(&self.texture);
        render_window.draw(&self.clerc_rc);
        render_window.draw(&self.magician_rc);
        render_window.draw(&self.ninja_rc);
        render_window.draw(&self.warrior_rc);
    }
}
