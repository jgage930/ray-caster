use anyhow::{Context, Result};
use std::fmt::{self};
use std::fs;

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

    /// Set pixel at x, y to draw color.
    fn set_pixel(&mut self, x: usize, y: usize) {
        self.buf[x][y] = self.draw_color.clone();
    }

    pub fn draw_rect(&mut self, top: usize, left: usize, color: &Color) {}
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
