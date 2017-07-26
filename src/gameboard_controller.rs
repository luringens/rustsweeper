//! Gameboard controller.

use piston::input::GenericEvent;
use Gameboard;
use gameboard::{CellState, BOARDSIZE};

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

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, MouseButton};
        
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;            
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            
            // Check that coordinates are inside the board.
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let cell_x = (x / size * 10.0) as usize;
                let cell_y = (y / size * 10.0) as usize;
                
                match self.gameboard.cells[cell_y][cell_x] {
                    CellState::HiddenBomb => { 
                        // GAME OVER
                        self.gameboard.cells[cell_y][cell_x] = CellState::Bomb;
                    },
                    CellState::HiddenBlank => {
                        let adjacent = self.count_adjacent_bombs(cell_y, cell_x);
                        match adjacent {
                            0 => self.gameboard.cells[cell_y][cell_x] = CellState::EmptyBlank,
                            _ => self.gameboard.cells[cell_y][cell_x] = CellState::EmptyNumber(adjacent as char),
                        }
                    }
                    _ => { },
                }
            }
        }
    }

    fn count_adjacent_bombs(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        for dy in -1..1 as i8 {
            for dx in -1..1 as i8 {
                let newx = x as i8 + dx;
                let newy = y as i8 + dy;
                if newx > 0 && newy > 0 && newx < BOARDSIZE as i8 - 1 && newy < BOARDSIZE as i8 - 1 &&
                    (newx != 0 || newy != 0) {
                    let celltype = self.gameboard.cells[newx as usize][newy as usize];
                    count += match celltype {
                        CellState::Bomb | CellState::HiddenBomb => 1,
                        _ => 0
                    }
                }
            }
        }
        count
    }
}