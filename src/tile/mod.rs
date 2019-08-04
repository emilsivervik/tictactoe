use piston_window::*;

use crate::globals;

#[derive(Copy, Clone, PartialEq)]
pub enum TileState {
    Empty,
    Cross,
    Circle
}

#[derive(Copy, Clone)]
pub struct Tile {
    value: TileState,
    x: i8,
    y: i8,
}

impl Tile {
    pub fn new(_x: i8, _y: i8) -> Tile {
        Tile {
            value: TileState::Empty,
            x: _x,
            y: _y,
        }
    }

    pub fn set_value(&mut self, new_value: TileState) {
        if self.value == TileState::Empty {
            self.value = new_value;
        }
    }

    pub fn reset(&mut self) {
        self.value = TileState::Empty;
    }

    pub fn can_set(&mut self) -> bool {
        self.value == TileState::Empty
    }

    pub fn is_value(&mut self, _inc_value: TileState) -> bool {
        self.value == _inc_value
    }

    pub fn render<G: Graphics>(&mut self, _args: &RenderArgs, _context: &Context, _gl: &mut G) {
        let x_position = globals::TILE_SIZE * self.x as f64;
        let y_position = globals::TILE_SIZE * self.y as f64;
        let square = rectangle::square(x_position, y_position, globals::TILE_SIZE);

        match self.value {
            TileState::Empty => {}
            TileState::Cross => {
                let transform = _context.transform.trans(0.0, 0.0);
                rectangle(globals::RED, square, transform, _gl);    
            }
            TileState::Circle => {
                let transform = _context.transform.trans(0.0, 0.0);
                ellipse(globals::RED, square, transform, _gl);                    
            }
        };
    }

    pub fn calculate_tile_index(cursor_x_position:f64, cursor_y_position:f64) -> u8 {
        cursor_y_position as u8 * globals::TILES_HEIGHT as u8 + cursor_x_position as u8
    }
}