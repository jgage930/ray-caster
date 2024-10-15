pub mod map;
pub mod renderer;

/// An alias for the result type used by sdl2.
pub type SdlResult<T> = std::result::Result<T, String>;
