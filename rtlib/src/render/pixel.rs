use super::color::Color;

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Self {
        Pixel { x, y, color }
    }
}
