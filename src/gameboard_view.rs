//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use GameboardController;
use gameboard::BOARDSIZE;
use traits::Renderer;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: (f64, f64),
    /// Size of the gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the board.
    pub board_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the board.
    pub board_edge_radius: f64,
    /// Edge radius around the cells.
    pub cell_edge_radius: f64,
    /// Text color.
    pub text_color: Color,
    /// Cell corner rounding.
    pub cell_corner_rounding: f64,
    /// Padding within each cell.
    pub cell_padding: f64,
}

impl GameboardViewSettings {
    /// Creates a new gameboard view setting.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: (0.0, 0.0),
            size: 600.0,
            background_color: [0.82, 0.9, 0.87, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            board_edge_color: [0.3, 0.3, 0.5, 1.0],
            cell_edge_color: [0.69, 0.76, 0.73, 1.0],
            cell_edge_radius: 1.0,
            cell_corner_rounding: 10.0,
            cell_padding: 7.0,
            text_color: [0.0, 0.0, 0.1, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
    /// The gameboard controller
    pub controller: GameboardController,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings, controller: GameboardController) -> GameboardView {
        GameboardView {
            settings: settings,
            controller: controller,
        }
    }
}

impl Renderer for GameboardView {
    /// Draw gameboard.
    fn draw<G: Graphics, C>(&self, glyphs: &mut C, c: &Context, g: &mut G)
        where C: CharacterCache<Texture = G::Texture>
    {
        use graphics::{Line, Rectangle, Image, Transformed};
        use gameboard::CellState::*;

        let settings = &self.settings;
        let board_rect = [settings.position.0, settings.position.1, settings.size, settings.size];

        // Draw background
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw cell borders.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        for i in 1..BOARDSIZE {
            let x = settings.position.0 + i as f64 / BOARDSIZE as f64 * settings.size;
            let y = settings.position.1 + i as f64 / BOARDSIZE as f64 * settings.size;
            let x2 = settings.position.0 + settings.size;
            let y2 = settings.position.1 + settings.size;

            let vline = [x, settings.position.1, x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position.0, y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // Draw each cell
        let cell_size = settings.size / BOARDSIZE as f64;
        let text_image = Image::new_color(settings.text_color);
        for y in 0..BOARDSIZE {
            for x in 0..BOARDSIZE {
                let color = match self.controller.gameboard.cells[y][x] {
                    HiddenBlank | HiddenBomb => [0.161, 0.31, 0.427, 1.0],
                    EmptyBlank | EmptyNumber(_) => [0.01, 0.52, 0.59, 1.0],
                    Bomb => [1.0, 0.0, 0.247, 1.0],
                    FlaggedBomb | FlaggedBlank => [0.1, 1.0, 0.1, 1.0],
                };
                let xpos = settings.position.0 + (x as f64) * (cell_size as f64) +
                           settings.cell_padding;
                let ypos = settings.position.1 + (y as f64) * (cell_size as f64) +
                           settings.cell_padding;
                let cell_rect = [xpos,
                                 ypos,
                                 cell_size - settings.cell_padding * 2.0,
                                 cell_size - settings.cell_padding * 2.0];
                let cell_rect_2 = [xpos,
                                   ypos - 5.0,
                                   cell_size - settings.cell_padding * 2.0,
                                   cell_size - settings.cell_padding * 2.0];

                match self.controller.gameboard.cells[y][x] {
                    HiddenBlank | HiddenBomb => {
                        Rectangle::new_round([0.01, 0.52, 0.59, 1.0],
                                             settings.cell_corner_rounding)
                            .draw(cell_rect, &c.draw_state, c.transform, g);
                        Rectangle::new_round([0.01, 0.71, 0.81, 1.0],
                                             settings.cell_corner_rounding)
                            .draw(cell_rect_2, &c.draw_state, c.transform, g);
                    }
                    _ => {
                        Rectangle::new_round(color, settings.cell_corner_rounding)
                            .draw(cell_rect, &c.draw_state, c.transform, g)
                    }
                };

                if let EmptyNumber(num) = self.controller.gameboard.cells[y][x] {
                    let character = glyphs.character(34, num);
                    let ch_x = xpos + character.left() + 13.0;
                    let ch_y = ypos - character.top() + 33.0;
                    text_image.draw(character.texture,
                                    &c.draw_state,
                                    c.transform.trans(ch_x, ch_y),
                                    g);
                }
            }
        }
    }
}
