use std::fmt;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    const RED: Self = Color { r: 255, g: 0, b: 0 };
    const GREEN: Self = Color { r: 0, g: 255, b: 0 };
    const BLUE: Self = Color { r: 0, g: 0, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn as_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

pub struct FrameBuffer {
    buf: Vec<Vec<Color>>,
}
