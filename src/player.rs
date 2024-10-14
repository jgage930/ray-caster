use crate::{
    map::{Map, Rect},
    utils::FloatRange,
    Drawable, FrameBuffer,
};
use std::f32::consts::PI;

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
            dir: PI / 4.,
            fov: PI / 3.,
            rect: Rect::new(x, x, PLAYER_SIZE, PLAYER_SIZE),
        }
    }

    /// Cast a single ray
    /// Draw ray to buffer.  to be removed later.
    pub fn cast_ray(&self, angle: f32, map: &Map, buf: &mut FrameBuffer) {
        let player_x = self.x as f32;
        let player_y = self.y as f32;

        let range = FloatRange::new(0., 10_000., map.tile_size() as f32);
        for c in range {
            let x = player_x + c * angle.cos();
            let y = player_y + c * angle.sin();

            let index_x = (x / map.tile_size() as f32).floor() as usize;
            let index_y = (y / map.tile_size() as f32).floor() as usize;
            if map.cells()[index_x][index_y] != ' ' {
                break;
            }

            buf.draw_pixel(x.floor() as usize, y.floor() as usize);
        }
    }

    /// Cast 512 rays within the field of view.
    pub fn cast_rays(&self, map: &Map, buf: &mut FrameBuffer) {
        let start = self.dir - (self.fov / 2.);
        let end = self.dir + (self.fov / 2.);
        // Calculate delta angle so 512 rays will be cast
        let d_a = self.fov / 512.;

        let range = FloatRange::new(start, end, d_a);
        for a in range {
            self.cast_ray(a, map, buf);
        }
    }
}

impl Drawable for Player {
    fn draw(&self, buf: &mut crate::FrameBuffer) {
        buf.draw_rect(&self.rect);
    }
}
