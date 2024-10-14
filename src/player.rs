use crate::{map::Rect, Drawable};

const PLAYER_SIZE: usize = 32;

pub struct Player {
    x: usize,
    y: usize,
    rect: Rect,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            rect: Rect::new(x, x, PLAYER_SIZE, PLAYER_SIZE),
        }
    }
}

impl Drawable for Player {
    fn draw(&self, buf: &mut crate::FrameBuffer) {
        buf.draw_rect(&self.rect);
    }
}
