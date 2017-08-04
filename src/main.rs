#![deny(missing_docs)]

//! A minesweeper game.

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use std::cell::RefCell;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, TextureSettings};
use opengl_graphics::glyph_cache::GlyphCache;

use gameboard::Gameboard;
use gameboard_controller::GameboardController;
use gameboard_view::*;
use traits::*;
use mainmenu::*;

mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod mainmenu;
mod traits;

fn main() {
    let opengl = OpenGL::V4_4;
    let settings = WindowSettings::new("Sudoku", [600; 2])
        .opengl(opengl)
        .samples(2)
        .resizable(false)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/Roboto-Bold.ttf", texture_settings)
        .expect("Could not load font");

    let mainmenu = MainMenu::new(MainMenuSettings::new());

    let mut renderer: Cell<Renderer> = RefCell::new(&mainmenu);
    let mut eventhandler: Cell<EventHandler> = RefCell:new(&mainmenu);

    while let Some(e) = events.next(&mut window) {
        eventhandler.event(gameboard_view.settings.position,
                                   gameboard_view.settings.size,
                                   &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                graphics::clear([0.44, 0.56, 0.64, 1.0], g);
                renderer.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}
