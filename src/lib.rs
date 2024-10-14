pub mod map;
pub mod player;
pub mod utils;

use anyhow::{Context, Result};
use map::{Point, Ray, Rect};
use std::{fmt, fs};

#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const RED: Self = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Color { r: 0, g: 0, b: 255 };
    pub const BLACK: Self = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn as_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} ", self.r, self.g, self.b)
    }
}

pub trait Drawable {
    fn draw(&self, buf: &mut FrameBuffer);
}

#[derive(Debug)]
pub struct FrameBuffer {
    width: usize,
    height: usize,
    buf: Vec<Vec<Color>>,
    draw_color: Color,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize, color: &Color) -> Self {
        let buf = vec![vec![color.clone(); width]; height];
        Self {
            width,
            height,
            buf,
            draw_color: Color::GREEN,
        }
    }

    /// Set the color to draw to the buffer with.
    pub fn set_draw_color(&mut self, color: Color) {
        self.draw_color = color;
    }

    /// Set pixel at x, y to draw color.
    pub fn draw_pixel(&mut self, x: usize, y: usize) {
        self.buf[x][y] = self.draw_color.clone();
    }

    pub fn draw<T: Drawable>(&mut self, object: &T) {
        object.draw(self);
    }

    pub fn draw_rect(&mut self, rect: &Rect) {
        let bottom = rect.top + rect.height;
        let right = rect.left + rect.width;

        for y in rect.top..bottom {
            for x in rect.left..right {
                self.draw_pixel(x, y);
            }
        }
    }

    pub fn render_3d(&mut self, rays: &Vec<Ray>) {
        for (x, ray) in rays.iter().enumerate() {
            // Tile size * screen height
            let height = 32. * 512. / ray.length();
            let line = VerticalLine::new(x, height as usize);

            self.draw(&line);
        }
    }
}

#[derive(Debug)]
struct VerticalLine {
    x: usize,
    length: usize,
}

impl VerticalLine {
    pub fn new(x: usize, length: usize) -> Self {
        Self { x, length }
    }
}

impl Drawable for VerticalLine {
    fn draw(&self, buf: &mut FrameBuffer) {
        let horizon = 256;

        let top = horizon - (self.length / 2);
        let bottom = horizon + (self.length / 2);

        for y in top..bottom {
            buf.draw_pixel(y, self.x)
        }
    }
}

/// Save a frame buffer to a ppm image.
/// # Arguments
/// * path - the path to save the image
/// * buf - the image buffer to write
pub fn save_ppm(path: &str, buffer: &FrameBuffer) -> Result<()> {
    let header = format!("P3\n{} {}\n255\n", buffer.width, buffer.height);

    let mut rows = String::new();
    for row in buffer.buf.iter() {
        let mut row_str = String::new();
        for color in row.iter() {
            row_str.push_str(&color.to_string());
        }
        row_str.push_str("\n");

        rows.push_str(&row_str);
    }

    let mut ppm = String::new();
    ppm.push_str(&header);
    ppm.push_str(&rows);

    fs::write(path, ppm).context("Failed to write ppm file")?;

    Ok(())
}
