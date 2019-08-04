use crate::globals;
use piston_window::*;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum InputKey {
    Pressed,
    Released,
}

#[derive(Clone, Eq, PartialEq)]
pub struct InputState {
    state: InputKey,
    served: bool,
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            state: InputKey::Released,
            served: false,
        }
    }
}

const MOUSE_POSTFIX: &str = "_mouse";
const KEYBOARD_POSTFIX: &str = "_keyboard";

#[derive(Clone, Default)]
pub struct InputHandler {
    pub cursor_position: (f64, f64),
    mouse_hash: HashMap<String, InputState>,
    pressed: bool,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            cursor_position: (0.0, 0.0),
            mouse_hash: HashMap::new(),
            pressed: false,
        }
    }

    pub fn set_cursor_position(&mut self, _cursor_positin: &[f64; 2]) {
        let [x_position, y_position] = *_cursor_positin;
    
        if x_position > 0.0 && y_position > 0.0 {
            let x_tile: f64 = ::math::round::floor(x_position / globals::TILE_SIZE, 0 );
            let y_tile: f64 = ::math::round::floor(y_position / globals::TILE_SIZE, 0 );

            self.cursor_position = (x_tile, y_tile)
        }
    }

    pub fn set_mouse_button(&mut self, _pressed_button: &MouseButton, _current_value: InputKey) {
        let new_state = InputState {
            state: _current_value,
            served: false,
        };

        let defer_press: MouseButton = *_pressed_button;
        let int_press_button: u32 = defer_press.into();
        let hash_key: String = int_press_button.to_string() + MOUSE_POSTFIX;

        self.mouse_hash.insert(hash_key, new_state);
    }

    pub fn set_keyboard_button(&mut self, _pressed_key: &Key, _current_value: InputKey) {
        let new_state = InputState {
            state: _current_value,
            served: false,
        };

        let defer_press: Key = *_pressed_key;
        let int_press_button: u32 = defer_press.into();
        let hash_key: String = int_press_button.to_string() + KEYBOARD_POSTFIX;

        self.mouse_hash.insert(hash_key, new_state);
    }

    fn is_pressed(&mut self, _pressed_button: &String) -> bool {
        let val = self.mouse_hash.get_mut(_pressed_button);

        match val {
            None => false,
            Some(val) => val.state == InputKey::Pressed && !val.served,
        }
    }

    pub fn is_keyboard_pressed(&mut self, _pressed_button: &Key) -> bool {
        let defer_press = *_pressed_button;
        let int_press_button: u32 = defer_press.into();
        let hash_key: String = int_press_button.to_string() + KEYBOARD_POSTFIX;

        self.is_pressed_and_changed(&hash_key)
    }

    pub fn is_mouse_pressed(&mut self, _pressed_button: &MouseButton) -> bool {
        let defer_press = *_pressed_button;
        let int_press_button: u32 = defer_press.into();
        let hash_key: String = int_press_button.to_string() + MOUSE_POSTFIX;

        self.is_pressed_and_changed(&hash_key)
    }

    pub fn is_pressed_and_changed(&mut self, _pressed_keyboard_button: &String) -> bool {
        if self.is_pressed(_pressed_keyboard_button) {
            let _last_state = self
                .mouse_hash
                .remove(_pressed_keyboard_button)
                .unwrap_or_default();

            let new_state = InputState {
                served: true,
                .._last_state
            };
            self.mouse_hash
                .insert(_pressed_keyboard_button.to_string(), new_state);
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, input_event: &Input) {
        match input_event {
            Input::Button(button_arguments) => {
                let state = if button_arguments.state == ButtonState::Press { InputKey::Pressed } else { InputKey::Released };
                match button_arguments.button {
                    Button::Keyboard(keyboard_key) => {
                        self.set_keyboard_button(&keyboard_key, state);
                    },
                    Button::Mouse(mouse_key) => {
                        self.set_mouse_button(&mouse_key, state);
                    },
                    _ => {}
                }
            },
            Input::Move(move_args) => {
                
                match move_args {
                    Motion::MouseCursor(mouse_position) => {
                        self.set_cursor_position(&mouse_position);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}