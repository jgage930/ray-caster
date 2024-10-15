pub mod map;
pub mod renderer;

use renderer::Renderer;

/// An alias for the result type used by sdl2.
pub type SdlResult<T> = std::result::Result<T, String>;

// A trait for anything that can be drawn my the renderer.
pub trait Drawable {
    fn draw(&self, renderer: &mut Renderer);
}
