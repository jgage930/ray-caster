use crate::{
    map::{Map, Rect},
    Drawable, FrameBuffer,
};

const PLAYER_SIZE: usize = 32;

pub struct Player {
    x: usize,
    y: usize,

    /// Direction player is looking in.
    dir: f32,
    /// Field of view.
    fov: f32,

    rect: Rect,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            dir: 3.14 / 4.,
            rect: Rect::new(x, x, PLAYER_SIZE, PLAYER_SIZE),
        }
    }

    /// Cast a single ray
    /// Draw ray to buffer.  to be removed later.
    pub fn cast_ray(&self, map: &Map, buf: &mut FrameBuffer) {
        let player_x = self.x as f32;
        let player_y = self.y as f32;

        for c in (0..1000).map(|x| x as f32 * 0.5) {
            let x = player_x + c * self.dir.cos();
            let y = player_y + c * self.dir.sin();

            let index_x = (x / map.tile_size() as f32).floor() as usize;
            let index_y = (y / map.tile_size() as f32).floor() as usize;
            if map.cells()[index_x][index_y] != ' ' {
                break;
            }

            println!("float vals x: {} y: {}", x, y);

            buf.draw_pixel(x.floor() as usize, y.floor() as usize);
        }
    }
}

impl Drawable for Player {
    fn draw(&self, buf: &mut crate::FrameBuffer) {
        buf.draw_rect(&self.rect);
    }
}
