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
    pub settings: GameboardViewSettings,
    cursor_pos: [0.0; 2],
}

impl MainMenu {
    /// Creates a new main menu object view.
    pub fn new(settings: MainMenuSettings) -> MainMenu {
        MainMenu { settings: settings }
    }

    pub fn event<E: GenericEvent>(&mut self, offset: (f64, f64), size: f64, e: &E) {
        use piston::input::{Button, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        // Left click
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            if  {
                self.open_cell(pos.0, pos.1);
            }
        }
    }

    pub fn draw<G: Graphics, C>(&self,
                                controller: &GameboardController,
                                glyphs: &mut C,
                                c: &Context,
                                g: &mut G)
        where C: CharacterCache<Texture = G::Texture>
    {
        // Title
        let text_image = Image::new_color(settings.text_color);
        for letter in "Rustsweeper!".chars().enumerate() {
            let character = glyphs.ch character(34, num);
            let ch_x = xpos + character.left() + 13.0;
            let ch_y = ypos - character.top() + 33.0;
            text_image.draw(character.texture,
                            &c.draw_state,
                            c.transform.trans(ch_x, ch_y),
                            g);
        }
    }
}

