use crate::engine::context::GameContext;
use crate::engine::Drawable;
use crate::engine::SdlResult;

use anyhow::Result;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub struct Renderer {
    pub canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Self> {
        let canvas = window.into_canvas().build()?;
        Ok(Self { canvas })
    }

    pub fn draw(&mut self, ctx: &GameContext) -> SdlResult<()> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        ctx.map.draw(&mut self.canvas)?;
        ctx.player.draw(&mut self.canvas)?;

        if let Some(rays) = &ctx.rays {
            for ray in rays {
                ray.draw(&mut self.canvas)?;
            }
        }

        self.canvas.present();

        Ok(())
    }
    
    pub fn render_3d(&mut self, window_height: i32, ctx: &GameContext) -> SdlResult<()> {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RED);
        let horizon = window_height / 2;

        if let Some(rays) = &ctx.rays {
            for (x, ray) in rays.iter().enumerate() {
                let line_height = (1. / ray.length()) * (window_height * 64) as f32; 
                println!("{line_height}");

                let top = (x as i32, horizon - (line_height / 2.) as i32);
                let bottom = (x as i32, horizon + (line_height / 2.) as i32);

                self.canvas.draw_line(
                    top, 
                    bottom 
                )?;
            }        
        }

        self.canvas.present();

        Ok(())
    }
}
