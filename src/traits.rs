use graphics::{Context, Graphics};
use graphics::character::CharacterCache;
use piston::input::GenericEvent;
use gameboard_controller::GameboardController;

pub trait Renderer {
    fn draw<G: Graphics, C>(&self,
                                controller: &GameboardController,
                                glyphs: &mut C,
                                c: &Context,
                                g: &mut G)
        where C: CharacterCache<Texture = G::Texture>;
}

pub trait EventHandler {
    fn event<E: GenericEvent>(&mut self, offset: (f64, f64), size: f64, e: &E);
}
