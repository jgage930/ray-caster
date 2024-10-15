use anyhow::Result;
use sdl2::rect::Rect;

type Cells = Vec<Vec<char>>;

pub struct Map {
    tile_size: u32,
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
        todo!()
    }

    pub fn tile_size(&self) -> u32 {
        self.tile_size
    }

    /// Get a copy of the maps cells.
    pub fn cells(&self) -> Cells {
        self.cells.clone()
    }

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
}
