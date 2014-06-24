use rsfml::graphics::{RenderWindow};

use game_state::Viewable;
use resource_loader::ResourceLoader;

pub struct Game<'s> {
    empty: u32
}

impl<'s> Game<'s> {
    pub fn new(resource_loader: &'s ResourceLoader) -> Game<'s> {
        Game { empty: 0}
    }
}

impl<'s> Viewable for Game<'s> {
    fn update(&self, render_window: &mut RenderWindow) -> Option<i32> {
        None
    }

    fn draw(&self, render_window: &mut RenderWindow) -> () {

    }
}