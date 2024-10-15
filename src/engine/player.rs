use sdl2::{pixels::Color, rect::Rect};

use crate::engine::Drawable;

const SIZE: u32 = 32;
const COLOR: Color = Color::GREEN;

#[derive(Debug, Clone)]
pub struct Player {
    rect: Rect,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
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
