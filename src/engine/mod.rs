pub mod context;
pub mod map;
pub mod player;
pub mod renderer;
pub mod cast;

use sdl2::render::WindowCanvas;

/// An alias for the result type used by sdl2.
pub type SdlResult<T> = std::result::Result<T, String>;

// A trait for anything that can be drawn my the renderer.
pub trait Drawable {
    fn draw(&self, canvas: &mut WindowCanvas) -> SdlResult<()>;
}
