use rsfml::graphics::{RenderWindow};

pub struct GameState {
    view: Box<Viewable>,
    enabled: bool
}

pub trait Viewable {
    fn update(&self, render_window: &mut RenderWindow) -> Option<i32>; /* return new state index or nothing*/
    fn draw(&self, render_window: &mut RenderWindow) -> ();
}

impl GameState {
    pub fn new(view: Box<Viewable>) -> GameState {
        GameState {
            view: view,
            enabled: false
        }
    }

    pub fn update(&self, render_window: &mut RenderWindow) -> Option<i32> {
        self.view.update(render_window)
    }

    pub fn draw(&self, render_window: &mut RenderWindow) -> () {
        self.view.draw(render_window);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) -> () {
        self.enabled = enabled;
    }
}
