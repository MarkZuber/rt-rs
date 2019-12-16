use crate::render::Color;
use crate::textures::perlin_noise;
use crate::textures::Texture;
use crate::{Point2, Vector3};
use std::sync::Arc;

pub struct NoiseTexture {
    interpolate: bool,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(interpolate: bool, scale: f32) -> Arc<Box<NoiseTexture>> {
        Arc::new(Box::new(NoiseTexture { interpolate, scale }))
    }
}

impl Texture for NoiseTexture {
    fn get_value(&self, _uv_coords: Point2<f32>, p: Vector3<f32>) -> Color {
        Color::one().multiply_by_scalar(perlin_noise(self.scale * p, self.interpolate))
    }
}
