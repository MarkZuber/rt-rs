use crate::render::Color;

pub struct PixelData {
    _x: u32,
    _y: u32,
    _color: Color,
    max_depth_reached: u32,
}

impl PixelData {
    pub fn new(x: u32, y: u32) -> PixelData {
        PixelData {
            _x: x,
            _y: y,
            _color: Color::new(0.0, 0.0, 0.0),
            max_depth_reached: 0,
        }
    }

    pub fn set_depth(&mut self, depth: u32) {
        if depth > self.max_depth_reached {
            self.max_depth_reached = depth;
        }
    }
}
