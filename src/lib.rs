use anyhow::{Context, Result};
use std::{convert::From, fmt, fs};

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

    pub fn draw<T: Drawable>(&mut self, object: T) {
        object.draw(self);
    }

    pub fn draw_rect(&mut self, rect: &Rect) {
        let bottom = rect.top + rect.height;
        let right = rect.left + rect.width;

        for y in rect.top..bottom {
            for x in rect.left..right {
                self.draw_pixel(y, x);
            }
        }
    }
}

#[derive(Debug)]
pub struct Rect {
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(top: usize, left: usize, width: usize, height: usize) -> Self {
        Self {
            top,
            left,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    tile_size: usize,
    width: usize,
    height: usize,
    walls: Vec<Rect>,
}

impl Map {
    /// Parse a string into a vec<vec<char>>
    fn parse_cells(val: &str) -> Vec<Vec<char>> {
        let mut cells: Vec<Vec<char>> = Vec::new();

        let lines: Vec<&str> = val.split('\n').collect();
        for line in lines.iter() {
            let row: Vec<char> = line.trim().chars().collect();

            cells.push(row);
        }

        cells
    }

    pub fn new(tile_size: usize, map: &str) -> Self {
        // Split str into a Vec<Vec<char>>
        let cells = Self::parse_cells(map);

        let width = &cells[0].len();
        let height = &cells.len();

        // Create the rectangles
        let mut walls = Vec::new();
        for x in 0..*width {
            for y in 0..*height {
                let c = &cells[x][y];
                if *c == ' ' {
                    continue;
                }

                let top = y * tile_size;
                let left = x * tile_size;

                let rect = Rect::new(top, left, tile_size, tile_size);
                walls.push(rect);
            }
        }

        Self {
            tile_size,
            width: *width,
            height: *height,
            walls,
        }
    }

    pub fn into_buffer(&self) -> FrameBuffer {
        FrameBuffer::new(
            self.tile_size * self.width,
            self.tile_size * self.height,
            &Color::BLACK,
        )
    }
}

impl Drawable for Map {
    fn draw(&self, buf: &mut FrameBuffer) {
        for wall in &self.walls {
            buf.draw_rect(wall);
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
