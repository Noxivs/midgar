#![crate_id = "github.com/Noxivs/midgar#midgar:0.1"]
#![desc = "rust version of midgar project (nami-doc's project in C)"]
#![crate_type = "bin"]

#![feature(globs)]

extern crate rsfml;

use std::cell::RefCell;

use rsfml::window::{ContextSettings, VideoMode, Close};
use rsfml::graphics::{RenderWindow};

pub mod event_handler;
pub mod game_loop;
pub mod game_state;
pub mod resource_loader;

#[path="./menu/menu.rs"]
pub mod menu;

#[path="./character/character.rs"]
pub mod character;

#[path="./game/game.rs"]
pub mod game;

fn create_window() -> RenderWindow {
    let setting = ContextSettings::default();

    match RenderWindow::new(VideoMode::new_init(512, 512, 32),
        "Midgar",
        Close,
        &setting) {
        Some(mut render_window) => {
            render_window.set_framerate_limit(60);
            render_window.set_mouse_cursor_visible(false);
            render_window 
        },
        None => fail!("Cannot create a new Render Window.")
    }
}

fn load_resources() -> resource_loader::ResourceLoader {
    let mut resource_loader = resource_loader::ResourceLoader::new();
    if 
        resource_loader.load_texture("../../res/welcome.jpg".to_string()) == false      ||  //00
        resource_loader.load_texture("../../res/texture.jpg".to_string()) == false      ||  //01
        resource_loader.load_texture("../../res/clerc.gif".to_string()) == false        ||  //02
        resource_loader.load_texture("../../res/magician.gif".to_string()) == false     ||  //03
        resource_loader.load_texture("../../res/ninja.gif".to_string()) == false        ||  //04
        resource_loader.load_texture("../../res/warrior.gif".to_string()) == false          //05
    {
        fail!("Error load textures");
    }
    resource_loader
}

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}


fn main () -> () {
    let resource_loader = load_resources();
    let render_window = create_window();
    let mut game_loop = game_loop::GameLoop::new(render_window, &resource_loader);

    let mut first_state = game_state::GameState::new(
            box menu::Menu::new(&resource_loader) as Box<game_state::Viewable>
        );

    first_state.set_enabled(true);

    game_loop.push_game_state(RefCell::new(first_state));

    game_loop.push_game_state(RefCell::new(game_state::GameState::new(
            box character::Character::new(&resource_loader) as Box<game_state::Viewable>
        )));

    game_loop.push_game_state(RefCell::new(game_state::GameState::new(
            box game::Game::new(&resource_loader) as Box<game_state::Viewable>
        )));

    game_loop.run();
}