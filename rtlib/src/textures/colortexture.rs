use crate::render::Color;
use crate::textures::Texture;
use crate::{Point2, Vector3};

pub struct ColorTexture {
    color: Color,
}

impl ColorTexture {
    pub fn new(r: f32, g: f32, b: f32) -> ColorTexture {
        ColorTexture {
            color: Color::new(r, g, b),
        }
    }
}

impl Texture for ColorTexture {
    fn get_value(&self, _uv_coords: Point2<f32>, _p: Vector3<f32>) -> Color {
        self.color
    }
}
