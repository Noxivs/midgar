use rsfml::graphics::{RenderWindow, RectangleShape, Color};
use rsfml::window::{event};
use rsfml::system::{Vector2f};

use resource_loader::ResourceLoader;
use game_state::{GameState};

pub struct GameLoop<'s> {
	render_window: RenderWindow,
	clear_color: Color,
	resource_loader: &'s ResourceLoader,
	game_states: Vec<Box<GameState>>
}

impl<'s>  GameLoop<'s>  {
	pub fn new(render_window: RenderWindow, resource_loader: &'s ResourceLoader) -> GameLoop<'s> {
		GameLoop { 
			render_window: render_window,
			clear_color : Color::new_RGB(100, 100, 100),
			resource_loader: resource_loader,
			game_states: Vec::new()
		}
	}

	pub fn push_game_state(&mut self, game_state: Box<GameState>) -> () {
		self.game_states.push(game_state);
    }


	fn update(&mut self) -> () {
		loop {
			match self.render_window.poll_event() {
				event::Closed 	=> self.render_window.close(),
                event::NoEvent 	=> break,
				_ 				=> {}
			}
		}

		for game_state in self.game_states.iter() {
			game_state.update(&mut self.render_window);
		}
	}

	fn draw(&mut self) -> () {
		self.render_window.clear(&self.clear_color);

		for game_state in self.game_states.iter() {
			game_state.draw(&mut self.render_window);
		}

		self.render_window.display();
	}

	pub fn run(&mut self) -> () {
		while self.render_window.is_open() {
			self.update();
			self.draw();
		}
	}
}