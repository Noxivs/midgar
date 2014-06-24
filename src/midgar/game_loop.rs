
use std::cell::{RefCell, RefMut};

use rsfml::graphics::{RenderWindow, RectangleShape, Color};
use rsfml::window::{event};
use rsfml::system::{Vector2f};

use resource_loader::ResourceLoader;
use game_state::{GameState};

pub struct GameLoop<'s> {
    render_window: RenderWindow,
    clear_color: Color,
    resource_loader: &'s ResourceLoader,
    game_states: Vec<RefCell<GameState>>
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

    pub fn push_game_state(&mut self, game_state: RefCell<GameState>) -> () {
        self.game_states.push(game_state);
    }


    fn update(&mut self) -> () {

        loop {
            match self.render_window.poll_event() {
                event::Closed   => self.render_window.close(),
                event::NoEvent  => break,
                _               => {}
                }
        }

        for game_state in self.game_states.iter() {
            if game_state.borrow().deref().get_enabled() {
                match game_state.borrow_mut().deref_mut().update(&mut self.render_window) {
                    Some(index)  => {
                        self.game_states.get(index as uint).borrow_mut().deref_mut().set_enabled(true);
                    }
                    None => { }
                }
            }
        }
    }

    fn draw(&mut self) -> () {
        self.render_window.clear(&self.clear_color);

        for game_state in self.game_states.iter() {
            if game_state.borrow().deref().get_enabled() {
                game_state.borrow_mut().deref_mut().draw(&mut self.render_window);
            }
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