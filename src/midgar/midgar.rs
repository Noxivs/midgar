#![crate_id = "github.com/Noxivs/midgar#midgar:0.1"]
#![desc = "rust version of midgar project (nami-doc's project in C)"]
#![crate_type = "bin"]

extern crate rsfml;

use rsfml::window::{ContextSettings, VideoMode, Close};
use rsfml::graphics::{RenderWindow};

pub mod game_loop;
pub mod resource_loader;
pub mod game_state;

#[path="./menu/menu.rs"]
pub mod menu;

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
    if resource_loader.load_texture("../../res/welcome.jpg".to_string()) == false {
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
    game_loop.push_game_state(
        box game_state::GameState::new(
            box menu::Menu::new(&resource_loader) as Box<game_state::Viewable>
            ));

    game_loop.run();
}