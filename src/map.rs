use crate::{Color, Drawable, FrameBuffer};

#[derive(Debug)]
pub struct Rect {
    pub top: usize,
    pub left: usize,
    pub width: usize,
    pub height: usize,
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

        // Create the rectangle
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
