#[macro_use]
extern crate lazy_static;
extern crate find_folder;
extern crate piston_window;
extern crate sdl2_window;
extern crate math;

pub mod globals;
pub mod game;
pub mod inputhandler;
pub mod tile;
pub mod assets;

use piston_window::*;
use sdl2_window::Sdl2Window;

fn main() {
    let window_width = globals::TILES_WIDTH as f64 * globals::TILE_SIZE;
    let window_height = globals::TILES_HEIGHT as f64 * globals::TILE_SIZE;
    let window: PistonWindow<Sdl2Window> = WindowSettings::new("Tic Tac Toe", [window_width, window_height])
        .graphics_api(globals::OPEN_GL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut app = game::TicTacToe::new(window);
    app.init_gameloop();
}
