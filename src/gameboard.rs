//! Game board logic.

/// Size of the game board.
pub const BOARDSIZE: usize = 10;
pub const BOMBCOUNT: usize = 12;

#[derive(Copy,Clone)]
/// Represents the different cell states.
pub enum CellState {
    // Hidden blank cell.
    HiddenBlank,
    // Hidden bomb.
    HiddenBomb,
    // Empty blank cell.
    EmptyBlank,
    // Empty cell with number of adjacent bombs.
    EmptyNumber(char),
    // An exploding bomb!
    Bomb,
    // A flagged bomb.
    FlaggedBomb,
    // A flagged empty space. Mistakes were made.
    FlaggedBlank,
}

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    pub cells: [[CellState; BOARDSIZE]; BOARDSIZE],
}

impl Gameboard {
    /// Creates a new gameboard.
    pub fn new() -> Gameboard {
        use rand;
        use rand::Rng;

        let mut newcells: [[CellState; BOARDSIZE]; BOARDSIZE] = 
            [[CellState::HiddenBlank; BOARDSIZE]; BOARDSIZE];
        let mut rng = rand::thread_rng();
        for _ in 0..BOMBCOUNT {
            let x = rng.gen_range::<usize>(0, BOARDSIZE);
            let y = rng.gen_range::<usize>(0, BOARDSIZE);
            newcells[y][x] = CellState::HiddenBomb;
        }

        Gameboard {
            cells: newcells,
        }
    }
}