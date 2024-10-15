use anyhow::Result;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::fs;

use crate::engine::{Drawable, SdlResult};

type Cells = Vec<Vec<char>>;

#[derive(Debug, Clone)]
pub struct Map {
    tile_size: usize,
    width: usize,
    height: usize,
    cells: Cells,
    walls: Vec<Rect>,
}

impl Map {
    /// Create a Map from a path to a file
    /// The first line in the file must be the tile size.
    /// Followed by a height x width grid of chars.
    /// Any cell that is not space will be a wall.
    pub fn new(path: &str) -> Result<Self> {
        let file = fs::read_to_string(path)?;
        let lines: Vec<&str> = file
            .split('\n')
            .filter_map(|l| if l == "" { return None } else { Some(l.trim()) })
            .collect();

        let tile_size = &lines[0].trim().parse::<usize>()?;
        let map_str = &lines[1..];

        let width = &map_str[0].len();
        let height = &map_str.len();

        let cells = Self::parse_cells(map_str.to_vec());

        // Create the walls
        let mut walls = Vec::new();
        for x in 0..*width {
            for y in 0..*height {
                let c = &cells[x][y];
                if *c == ' ' {
                    continue;
                }

                let top = y * tile_size;
                let left = x * tile_size;

                let rect = Rect::new(
                    top as i32,
                    left as i32,
                    *tile_size as u32,
                    *tile_size as u32,
                );
                walls.push(rect);
            }
        }

        Ok(Self {
            tile_size: *tile_size,
            width: *width,
            height: *height,
            cells,
            walls,
        })
    }

    pub fn tile_size(&self) -> usize {
        self.tile_size
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Get a copy of the maps cells.
    pub fn cells(&self) -> Cells {
        self.cells.clone()
    }

    pub fn walls(&self) -> Vec<Rect> {
        self.walls.clone()
    }

    /// Parse a string into a vec<vec<char>>
    fn parse_cells(lines: Vec<&str>) -> Vec<Vec<char>> {
        let mut cells: Vec<Vec<char>> = Vec::new();

        for line in lines.iter() {
            let row: Vec<char> = line.trim().chars().collect();

            cells.push(row);
        }

        cells
    }
}

impl Drawable for Map {
    fn draw(&self, canvas: &mut WindowCanvas) -> SdlResult<()> {
        canvas.set_draw_color(Color::RED);

        for wall in &self.walls {
            canvas.draw_rect(*wall)?;
        }

        Ok(())
    }
}
