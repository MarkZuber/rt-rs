use crate::render::Color;
use crate::textures::Texture;
use crate::textures::{vector_perlin_noise, vector_perlin_turbulence, ThreadTexture};
use crate::{Point2, Vector3};
use std::sync::Arc;

pub enum VectorNoiseMode {
    DarkNoise,
    DarkTurbulence,
    Soft,
    Marble,
}

pub struct VectorNoiseTexture {
    mode: VectorNoiseMode,
    scale: f32,
}

impl VectorNoiseTexture {
    pub fn new(mode: VectorNoiseMode, scale: f32) -> ThreadTexture {
        Arc::new(Box::new(VectorNoiseTexture { mode, scale }))
    }
}

impl Texture for VectorNoiseTexture {
    fn get_value(&self, _uv_coords: Point2<f32>, p: Vector3<f32>) -> Color {
        match self.mode {
            VectorNoiseMode::Soft => {
                return Color::one()
                    .multiply_by_scalar(0.5 * (1.0 + vector_perlin_turbulence(self.scale * p)))
            }
            VectorNoiseMode::DarkNoise => {
                return Color::one().multiply_by_scalar(vector_perlin_noise(self.scale * p))
            }
            VectorNoiseMode::DarkTurbulence => {
                return Color::one().multiply_by_scalar(vector_perlin_turbulence(self.scale * p))
            }
            VectorNoiseMode::Marble => {
                return Color::one().multiply_by_scalar(
                    0.5 * (1.0 + (self.scale * p.z).sin() + (10.0 * vector_perlin_turbulence(p))),
                )
            }
        }
    }
}
