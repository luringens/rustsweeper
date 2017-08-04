use graphics::types::Color;
use graphics::{Context, Graphics, Text};
use graphics::character::CharacterCache;
use piston::input::GenericEvent;

use gameboard_controller::GameboardController;
use traits::*;

/// Stores main menu settings.
pub struct MainMenuSettings {
    /// Position from left-top corner.
    pub position: (f64, f64),
    /// Size of the main menu.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Text color.
    pub text_color: Color,
}

impl MainMenuSettings {
    /// Creates a new main menu settings object.
    pub fn new() -> MainMenuSettings {
        MainMenuSettings {
            position: (0.0, 0.0),
            size: 600.0,
            background_color: [0.82, 0.9, 0.87, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
        }
    }
}

pub struct MainMenu {
    pub settings: MainMenuSettings,
    cursor_pos: [f64; 2],
}

impl MainMenu {
    /// Creates a new main menu object view.
    pub fn new(settings: MainMenuSettings) -> MainMenu {
        MainMenu {
            settings: settings,
            cursor_pos: [0.0, 0.0],
        }
    }
}

impl EventHandler for MainMenu {
    fn event<E: GenericEvent>(&mut self, offset: (f64, f64), size: f64, e: &E) {
        use piston::input::{Button, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        // Left click
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            if true {
                //todo
            }
        }
    }
}

impl Renderer for MainMenu {
    fn draw<G: Graphics, C>(&self,
                                controller: &gameboard::GameboardController,
                                glyphs: &mut C,
                                c: &Context,
                                g: &mut G)
        where C: CharacterCache<Texture = G::Texture>
    {
        // Title
        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
            "Rustsweeper!",
            &mut glyphs,
            &c.draw_state,
            transform, g
        );
    }
}

