use piston_window::*;
use sdl2_window::*;

use crate::assets::Assets;
use crate::globals;
use crate::tile::{Tile, TileState};

const START_TEXT: &str = "Press E to start.";

enum GameState {
    Start,
    Playing,
    Gameover,
}

pub struct TicTacToe {
    board: [Tile; 9],
    current_winner: TileState,
    current_player: TileState,
    current_game_state: GameState,
    assets: Assets,
    window: PistonWindow<Sdl2Window>,
}

impl TicTacToe {
    pub fn new(mut _window: PistonWindow<Sdl2Window>) -> Self {
        TicTacToe {
            board: TicTacToe::init_board(),
            current_winner: TileState::Empty,
            current_player: TileState::Cross,
            current_game_state: GameState::Start,
            assets: Assets::new(&mut _window),
            window: _window,
        }
    }

    fn render(&mut self, event: &Event, render_args: &RenderArgs) {
        let window = &mut self.window;
        let game_state = &self.current_game_state;
        let assets = &mut self.assets;
        let board = &mut self.board;
        let current_player_state = self.current_player;
        let winner = self.current_winner;

        window.draw_2d(event, |context, gl, device| {
            clear(globals::WHITE, gl);
            match game_state {
                GameState::Start => {
                    assets.render_text(12, START_TEXT, [12.0, 12.0], &context, gl);

                }
                GameState::Playing => {
                    for tile in board.iter_mut() {
                        tile.render(render_args, &context, gl);
                    }
                    let player_turn_text = if current_player_state == TileState::Circle {
                        "Player O turn."
                    } else {
                        "Player X turn."
                    };
                    assets.render_text(12, player_turn_text, [12.0, 12.0], &context, gl);
                }
                GameState::Gameover => {
                    for tile in board.iter_mut() {
                        tile.render(render_args, &context, gl);
                    }
                    assets.render_text(12, "GAME FKN OVER MAN", [125.0, 50.0], &context, gl);

                    let player_winner = if winner == TileState::Circle {
                        "Player O Winner."
                    } else if winner == TileState::Cross {
                        "Player X Winner."
                    } else {
                        "It's a Draw!"
                    };
                    assets.render_text(12, player_winner, [125.0, 125.0], &context, gl);
                    assets.render_text(12, "Press R to reset.", [125.0, 250.0], &context, gl);
                }
            }
            assets.px_plus_font.factory.encoder.flush(device);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let mut input = globals::INPUT_HANDLER.lock().unwrap();
        match self.current_game_state {
            GameState::Start => {
                if input.is_keyboard_pressed(&Key::E) {
                    self.current_game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                if input.is_mouse_pressed(&MouseButton::Left) {
                    let (cursor_x_position, cursor_y_position) = input.cursor_position;
                    let index = Tile::calculate_tile_index(cursor_x_position, cursor_y_position);
                    self.press_tile(index);
                }
                self.evaluate_winning_conditions();
            }
            GameState::Gameover => {
                if input.is_keyboard_pressed(&Key::R) {
                    self.reset_board();
                }
            }
        }
    }

    fn reset_board(&mut self) {
        self.current_winner = TileState::Empty;
        self.current_player = TileState::Cross;
        self.current_game_state = GameState::Start;

        for tile in self.board.iter_mut() {
            tile.reset();
        }
    }

    fn matches_winning_conditions(
        &mut self,
        current_board: [Tile; 9],
        tile_tuple: (usize, usize, usize),
    ) {
        let (first, second, third) = tile_tuple;

        let compare_arr = [
            current_board[first],
            current_board[second],
            current_board[third],
        ];

        fn matching(current_board: [Tile; 3], player_value: TileState) -> TileState {
            let [mut first, mut second, mut third] = current_board;
            if first.is_value(player_value)
                && second.is_value(player_value)
                && third.is_value(player_value)
            {
                player_value
            } else {
                TileState::Empty
            }
        }

        let circle_row = matching(compare_arr, TileState::Circle);
        let cross_row = matching(compare_arr, TileState::Cross);

        if circle_row != TileState::Empty {
            self.current_winner = circle_row;
            self.current_game_state = GameState::Gameover;
        } else if cross_row != TileState::Empty {
            self.current_winner = cross_row;
            self.current_game_state = GameState::Gameover;
        }
    }

    fn evaluate_winning_conditions(&mut self) {
        for i in 0..3 {
            /* Column */
            let column_first = (i as i8 + globals::TILES_HEIGHT) as usize;
            let column_second = (i as i8 + globals::TILES_HEIGHT * 1) as usize;
            let column_third = (i as i8 + globals::TILES_HEIGHT * 2) as usize;
            let column_input_tuple: (usize, usize, usize) =
                (column_first, column_second, column_third);
            self.matches_winning_conditions(self.board, column_input_tuple);

            /* Row */
            let row_first = (i as i8 * globals::TILES_HEIGHT) as usize;
            let row_second = (i as i8 * globals::TILES_HEIGHT + 1) as usize;
            let row_third = (i as i8 * globals::TILES_HEIGHT + 2) as usize;
            let row_input_tuple: (usize, usize, usize) = (row_first, row_second, row_third);
            self.matches_winning_conditions(self.board, row_input_tuple);
        }

        /* Diagonal */
        {
            let first = 0 as usize;
            let second = 4 as usize;
            let third = 8 as usize;
            let diagonal_input_tuple: (usize, usize, usize) = (first, second, third);
            self.matches_winning_conditions(self.board, diagonal_input_tuple);
        }

        {
            let first = 2 as usize;
            let second = 4 as usize;
            let third = 6 as usize;
            let diagonal_input_tuple: (usize, usize, usize) = (first, second, third);
            self.matches_winning_conditions(self.board, diagonal_input_tuple);
        }

        /* Draw */
        let mut draw = true;
        for tile in self.board.iter_mut() {
            if tile.is_value(TileState::Empty) {
                draw = false;
            }
        }
        if draw {
            self.current_game_state = GameState::Gameover;
        }
    }

    pub fn init_gameloop(&mut self) {
        let mut events = Events::new(EventSettings::new());

        while let Some(event) = events.next(&mut self.window) {
            match event {
                Event::Input(input_event, _dt) => {
                    globals::INPUT_HANDLER.lock().unwrap().update(&input_event)
                }
                Event::Loop(loop_event) => match loop_event {
                    Loop::Render(render_arguments) => self.render(&event, &render_arguments),
                    Loop::Update(update_arguments) => self.update(&update_arguments),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn init_board() -> [Tile; 9] {
        let mut x = 0;
        let mut y = 0;

        const ARR_SIZE: usize = (globals::TILES_HEIGHT * globals::TILES_WIDTH) as usize;

        let tiles_array: [Tile; ARR_SIZE] = unsafe {
            let mut arr: [Tile; ARR_SIZE] = std::mem::uninitialized();
            for item in &mut arr[..] {

                let new_tile = Tile::new(x, y);
                std::ptr::write(item, new_tile);

                if x >= globals::TILES_WIDTH - 1 {
                    x = 0;
                    y = y + 1;
                } else {
                    x = x + 1;
                }
            }
            arr
        };

        tiles_array
    }

    fn press_tile(&mut self, index_pressed: u8) {
        if self.board[index_pressed as usize].can_set() {
            self.board[index_pressed as usize].set_value(self.current_player);
            self.current_player = if self.current_player == TileState::Circle {
                TileState::Cross
            } else {
                TileState::Circle
            }
        }
    }
}
