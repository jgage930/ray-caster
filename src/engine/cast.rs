use sdl2::rect::Point;
use sdl2::pixels::Color;

use crate::{engine::{player, Drawable}, utils::FloatRange};

use super::context::GameContext;


#[derive(Debug)]
pub struct Ray {
    start: Point,
    end: Point,
}

impl Ray {
    pub fn new(x_0: i32, y_0: i32, x_1: i32, y_1: i32) -> Self {
        Self {
            start: Point::new(x_0, y_0),
            end: Point::new(x_1, y_1),

        }
    }
}

impl Drawable for Ray {
    fn draw(&self, canvas: &mut sdl2::render::WindowCanvas) -> super::SdlResult<()> {
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(self.start, self.end)?;

        Ok(())
    }
}

pub fn cast_single_ray(ctx: &GameContext) -> Ray {
    let center = ctx.player.rect.center();

    let player_x = center.x() as f32;
    let player_y = center.y() as f32;

    let angle = ctx.player.looking_at;

    let range = FloatRange::new(0., 10_000., 0.5);
    for c in range {
        let x = player_x + c * angle.cos();
        let y = player_y + c * angle.sin();

        let index_x = (x / ctx.map.tile_size() as f32).floor() as usize;
        let index_y = (y / ctx.map.tile_size() as f32).floor() as usize;
        if ctx.map.cells()[index_x][index_y] != ' ' {
            let ray = Ray::new(
                player_x as i32,
                player_y as i32,
                x as i32,
                y as i32,
            );
            return ray;
        }
    }

    Ray::new(0, 0, 0, 0)
}