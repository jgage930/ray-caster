use sdl2::{pixels::Color, rect::Rect};
use std::f32::consts::PI;

use crate::engine::Drawable;

const SIZE: u32 = 32;
const COLOR: Color = Color::GREEN;

#[derive(Debug, Clone)]
pub struct Player {
    // Angle that the player is looking at in radians.
    pub looking_at: f32,
    // The field of view of the player in radians.
    pub fov: f32,

    pub rect: Rect,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            looking_at: 0.,
            fov: PI / 3.,
            rect: Rect::new(x, y, SIZE, SIZE),
        }
    }
}

impl Drawable for Player {
    fn draw(&self, canvas: &mut sdl2::render::WindowCanvas) -> super::SdlResult<()> {
        canvas.set_draw_color(COLOR);
        canvas.draw_rect(self.rect)?;        
        Ok(())
    }
}
