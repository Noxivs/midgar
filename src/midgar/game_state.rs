use rsfml::graphics::{RenderWindow};

pub struct GameState {
	view: Box<Viewable>
}

pub trait Viewable {
	fn update(&self, render_window: &mut RenderWindow) -> ();
	fn draw(&self, render_window: &mut RenderWindow) -> ();
}

impl GameState {
	pub fn new(view: Box<Viewable>) -> GameState {
		GameState {
			view: view
		}
	}

	pub fn update(&self, render_window: &mut RenderWindow) -> () {
		self.view.update(render_window);
	}

	pub fn draw(&self, render_window: &mut RenderWindow) -> () {
		self.view.draw(render_window);
	}
}
