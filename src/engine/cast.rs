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

    pub fn length(&self) -> f32 {
        let d_x = (self.start.x - self.end.x) as f32;
        let d_y = (self.start.y - self.end.y) as f32;

        (d_x.powf(2.) + d_y.powf(2.)).sqrt()
    }
}

impl Drawable for Ray {
    fn draw(&self, canvas: &mut sdl2::render::WindowCanvas) -> super::SdlResult<()> {
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(self.start, self.end)?;

        Ok(())
    }
}

pub fn cast_single_ray(angle: f32, ctx: &GameContext) -> Ray {
    let center = ctx.player.rect.center();

    let player_x = center.x() as f32;
    let player_y = center.y() as f32;

    let range = FloatRange::new(0., 1_000., ctx.map.tile_size() as f32);
    for c in range {
        let x = player_x + c * angle.cos();
        let y = player_y + c * angle.sin();

        let index_x = (x / ctx.map.tile_size() as f32).floor() as usize;
        let index_y = (y / ctx.map.tile_size() as f32).floor() as usize;
        
        let char_at = ctx.map.cells()[index_y][index_x];
        if char_at != ' ' {
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

pub fn cast_rays(ctx: &GameContext) -> Vec<Ray> {
    let mut rays = Vec::new();

    let dir = ctx.player.looking_at;
    let fov = ctx.player.fov;

    let start = dir - (fov / 2.);
    let end = dir + (fov / 2.);
    // Calculate delta angle so 512 rays will be cast
    let d_a = fov / 512.;

    let range = FloatRange::new(start, end, d_a);
    for a in range {
        rays.push(cast_single_ray(a, ctx));
    }

    rays
}