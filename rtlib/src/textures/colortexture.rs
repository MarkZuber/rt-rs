use crate::render::Color;
use crate::textures::Texture;
use crate::{Point2, Vector3};
use std::sync::Arc;

pub struct ColorTexture {
    color: Color,
}

impl ColorTexture {
    pub fn new(r: f32, g: f32, b: f32) -> Arc<Box<ColorTexture>> {
        Arc::new(Box::new(ColorTexture {
            color: Color::new(r, g, b),
        }))
    }
}

impl Texture for ColorTexture {
    fn get_value(&self, _uv_coords: Point2<f32>, _p: Vector3<f32>) -> Color {
        self.color
    }
}
