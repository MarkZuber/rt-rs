mod checkertexture;
mod colortexture;
mod imagetexture;
mod noisetexture;
mod perlin;
mod vectornoisetexture;
mod vectorperlin;

pub use self::checkertexture::CheckerTexture;
pub use self::colortexture::ColorTexture;
pub use self::imagetexture::ImageTexture;
pub use self::noisetexture::NoiseTexture;
pub use self::perlin::perlin_noise;
pub use self::vectornoisetexture::VectorNoiseTexture;
pub use self::vectorperlin::{vector_perlin_noise, vector_perlin_turbulence};

use crate::render::Color;
use crate::{Point2, Vector3};
use std::sync::Arc;

pub trait Texture: Sync {
    fn get_value(&self, uv_coords: Point2<f32>, p: Vector3<f32>) -> Color;
}

pub type ThreadTexture = Arc<Box<dyn Texture + Send>>;
