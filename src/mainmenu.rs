use graphics::types::Color;
use graphics::{Context, Graphics, text, Rectangle};
use graphics::character::CharacterCache;
use piston::input::GenericEvent;
use piston::input::keyboard::Key;
use graphics::Transformed;

use traits::*;
use state::State;

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
    /// Button box color.
    pub box_color: Color,
    /// Button font size.
    pub font_size_button: u32,
    /// Title font size.
    pub font_size_title: u32,
    /// Button text margin.
    pub button_text_margin: f64,
}

impl MainMenuSettings {
    /// Creates a new main menu settings object.
    pub fn new() -> MainMenuSettings {
        MainMenuSettings {
            position: (0.0, 0.0),
            size: 600.0,
            background_color: [0.82, 0.9, 0.87, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
            box_color: [0.01, 0.52, 0.59, 1.0],
            font_size_button: 32,
            font_size_title: 38,
            button_text_margin: 7.0,
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

    fn get_button_rect(&self, index: u8, windowwidth: f64, windowheight: f64) -> [f64; 4] {
        [windowwidth / 3.0,
         windowheight / 10.0 * (index) as f64 + 5.0,
         windowwidth / 3.0,
         windowheight / 10.0 as f64 - 10.0]
    }

    fn drawcenteredtext<G: Graphics, C>(&self,
                                        text: &str,
                                        index: u8,
                                        fontsize: u32,
                                        glyphs: &mut C,
                                        c: &Context,
                                        g: &mut G)
        where C: CharacterCache<Texture = G::Texture>
    {
        let rect = self.get_button_rect(index, c.get_view_size()[0], c.get_view_size()[1]);
        let width = glyphs.width(fontsize, text);
        let x = c.get_view_size()[0] / 2.0 - width / 2.0;
        let transform = c.trans(x, rect[1] + rect[3] / 2.0 + 10.0).transform;
        text::Text::new_color([0.0, 0.0, 0.0, 1.0], fontsize)
            .draw(text, glyphs, &c.draw_state, transform, g);
    }

    fn drawcenteredtextwithbox<G: Graphics, C>(&self,
                                               text: &str,
                                               index: u8,
                                               fontsize: u32,
                                               glyphs: &mut C,
                                               c: &Context,
                                               g: &mut G)
        where C: CharacterCache<Texture = G::Texture>
    {
        let rect = self.get_button_rect(index, c.get_view_size()[0], c.get_view_size()[1]);
        Rectangle::new_round(self.settings.box_color, 5.0)
            .draw(rect, &c.draw_state, c.transform, g);
        self.drawcenteredtext(text, index, fontsize, glyphs, c, g);
    }

    /// Returns the index of the clicked button. Probably.
    fn clicked_button(&self, mousepos: [f64; 2], windowsize: (f64, f64)) -> Option<u8> {
        let rect = self.get_button_rect(0, windowsize.0, windowsize.1);
        if mousepos[0] < rect[0] || mousepos[0] > rect[0] + rect[2] || mousepos[1] < 0.0 ||
           mousepos[1] > windowsize.1 {
            return None;
        }
        Some((mousepos[1] / (windowsize.1 / 10.0)) as u8)
    }
}

impl EventHandler for MainMenu {
    fn event<E: GenericEvent>(&mut self, size: (f64, f64), e: &E) -> State {
        use piston::input::{Button, MouseButton};

        let mut nextstate = State::MainMenu;

        // Save mouse position.
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        // Handle button clicks.
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            if let Some(index) = self.clicked_button(self.cursor_pos, size) {
                nextstate = match index {
                    4 => State::GameBoard,
                    5 => State::Exiting,
                    _ => nextstate,
                };
            }
        }

        // Exit when you press ESC.
        if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
            nextstate = State::Exiting;
        }

        nextstate
    }
}

impl Renderer for MainMenu {
    fn draw<G: Graphics, C>(&self, glyphs: &mut C, c: &Context, g: &mut G)
        where C: CharacterCache<Texture = G::Texture>
    {
        self.drawcenteredtext("Rustsweeper!",
                              1,
                              self.settings.font_size_title,
                              glyphs,
                              c,
                              g);
        self.drawcenteredtextwithbox("Start", 4, self.settings.font_size_button, glyphs, c, g);
        self.drawcenteredtextwithbox("Quit", 5, self.settings.font_size_button, glyphs, c, g);
    }
}
