//! Gameboard controller.

use piston::input::GenericEvent;
use piston::input::keyboard::Key;
use Gameboard;
use gameboard::{CellState, BOARDSIZE};
use traits::EventHandler;
use state::State;

/// Handles events for the game.
pub struct GameboardController {
    /// Stores the state of the board.
    pub gameboard: Gameboard,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Stores last mouse cursor position.
    cursor_pos: [f64; 2],
}

impl GameboardController {
    /// Crates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    fn open_cell(&mut self, x: usize, y: usize) {
        match self.gameboard.cells[y][x] {
            CellState::HiddenBomb => self.gameboard.cells[y][x] = CellState::Bomb,
            CellState::HiddenBlank => {
                let adjacent = self.count_adjacent_bombs(y, x);
                match adjacent {
                    0 => {
                        self.gameboard.cells[y][x] = CellState::EmptyBlank;
                        for dy in -1..2 as i8 {
                            for dx in -1..2 as i8 {
                                if self.is_valid_cell(x as i8 + dx, y as i8 + dy) {
                                    self.open_cell((x as i8 + dx) as usize,
                                                   (y as i8 + dy) as usize);
                                }
                            }
                        }
                    }
                    _ => {
                        self.gameboard.cells[y][x] = CellState::EmptyNumber((adjacent + 48) as char)
                    }
                }
            }
            _ => {}
        }
    }

    fn flag_cell(&mut self, x: usize, y: usize) {
        self.gameboard.cells[y][x] = match self.gameboard.cells[y][x] {
            CellState::HiddenBomb => CellState::FlaggedBomb,
            CellState::HiddenBlank => CellState::FlaggedBlank,
            CellState::FlaggedBomb => CellState::HiddenBomb,
            CellState::FlaggedBlank => CellState::HiddenBlank,
            _ => self.gameboard.cells[y][x],
        }
    }

    fn get_selected_cell(&self, size: f64) -> Option<(usize, usize)> {
        // Find coordinates relative to upper left corner.
        let x = self.cursor_pos[0];
        let y = self.cursor_pos[1];
        // Check that coordinates are inside the board.
        if x >= 0.0 && x < size && y >= 0.0 && y < size {
            let cell_x = (x / size * 10.0) as usize;
            let cell_y = (y / size * 10.0) as usize;
            return Some((cell_x, cell_y));
        }
        None
    }

    fn is_valid_cell(&self, x: i8, y: i8) -> bool {
        x >= 0 && x < BOARDSIZE as i8 && y >= 0 && y < BOARDSIZE as i8
    }

    fn count_adjacent_bombs(&self, x: usize, y: usize) -> u8 {
        use gameboard::CellState::*;
        let mut count: u8 = 0;
        for dy in -1..2 as i8 {
            for dx in -1..2 as i8 {
                let newx = x as i8 + dx;
                let newy = y as i8 + dy;
                if newx >= 0 && newy >= 0 && newx < BOARDSIZE as i8 && newy < BOARDSIZE as i8 &&
                   (dx != 0 || dy != 0) {
                    let celltype = self.gameboard.cells[newx as usize][newy as usize];
                    count += match celltype {
                        Bomb | HiddenBomb | FlaggedBomb => 1,
                        _ => 0,
                    }
                }
            }
        }
        count
    }
}

impl EventHandler for GameboardController {
    /// Handles events.
    fn event<E: GenericEvent>(&mut self, size: (f64, f64), e: &E) -> State {
        use piston::input::{Button, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        // Left click
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {

            // Check that coordinates are inside the board.
            if let Some(pos) = self.get_selected_cell(size.0) {
                self.open_cell(pos.0, pos.1);
            }
        }

        // Exit to main menu when you press ESC.
        if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
            return State::MainMenu;
        }

        // Right click
        if let Some(Button::Mouse(MouseButton::Right)) = e.press_args() {

            // Check that coordinates are inside the board.
            if let Some(pos) = self.get_selected_cell(size.0) {
                self.flag_cell(pos.0, pos.1);
            }
        }

        State::GameBoard
    }
}
