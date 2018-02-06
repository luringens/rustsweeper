#![deny(missing_docs)]

//! A minesweeper game.

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::{Window, WindowSettings};
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::{RenderEvent, CloseEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, TextureSettings};
use opengl_graphics::glyph_cache::GlyphCache;

use gameboard::Gameboard;
use gameboard_controller::GameboardController;
use gameboard_view::*;
use traits::*;
use mainmenu::*;
use state::State;

mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod mainmenu;
mod traits;
mod state;

fn main() {
    let opengl = OpenGL::V4_4;
    let settings = WindowSettings::new("Sudoku", [600; 2])
        .opengl(opengl)
        .samples(2);

    let clear_color = [0.82, 0.9, 0.87, 0.0];

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let glyphs = &mut GlyphCache::new("assets/Roboto-Bold.ttf", texture_settings)
        .expect("Could not load font");

    let mut mainmenu = MainMenu::new(MainMenuSettings::new());
    let gameboard = Gameboard::new();
    let gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let mut gameboard_view = GameboardView::new(gameboard_view_settings, gameboard_controller);

    let mut state = State::MainMenu;

    while let Some(e) = events.next(&mut window) {
        if e.close_args().is_some() {
            break;
        }
        match state {
            State::MainMenu => {
                state = mainmenu.event((window.size().width as f64, window.size().height as f64),
                                       &e);
                if let Some(args) = e.render_args() {
                    gl.draw(args.viewport(), |c, g| {
                        graphics::clear(clear_color, g);
                        mainmenu.draw(glyphs, &c, g);
                    });
                }
            }
            State::GameBoard => {
                state = gameboard_view.controller
                    .event((window.size().width as f64, window.size().height as f64),
                           &e);

                if let Some(args) = e.render_args() {
                    gl.draw(args.viewport(), |c, g| {
                        graphics::clear(clear_color, g);
                        gameboard_view.draw(glyphs, &c, g);
                    });
                }
            }
            State::Exiting => break,
        };
    }
}
