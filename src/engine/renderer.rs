use crate::engine::context::GameContext;
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

        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.draw_line((100, 100), (0, 0))?;
        self.canvas.draw_line((500, 300), (0, 0))?;

        self.canvas.present();

        Ok(())
    }
}
