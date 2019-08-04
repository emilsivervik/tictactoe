use crate::inputhandler;
use sdl2_window::OpenGL;
use std::sync::Mutex;


pub const TILE_SIZE: f64 = 100.0;
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub const TILES_HEIGHT: i8 = 3;
pub const TILES_WIDTH: i8 = 3;
pub const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

pub const WINDOW_WIDTH: f64 = TILES_WIDTH as f64 * TILE_SIZE;
pub const WINDOW_HEIGHT: f64 = TILES_HEIGHT as f64 * TILE_SIZE;

lazy_static! {
    pub static ref INPUT_HANDLER: Mutex<inputhandler::InputHandler> = Mutex::new(inputhandler::InputHandler::new());
}