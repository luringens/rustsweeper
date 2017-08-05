use graphics::{Context, Graphics};
use graphics::character::CharacterCache;
use piston::input::GenericEvent;
use state::State;

pub trait Renderer {
    fn draw<G: Graphics, C>(&self, glyphs: &mut C, c: &Context, g: &mut G)
        where C: CharacterCache<Texture = G::Texture>;
}

pub trait EventHandler {
    fn event<E: GenericEvent>(&mut self, size: (f64, f64), e: &E) -> State;
}
