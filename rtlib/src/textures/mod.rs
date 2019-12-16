mod checkertexture;
mod colortexture;
mod noisetexture;
mod perlin;

pub use self::checkertexture::CheckerTexture;
pub use self::colortexture::ColorTexture;
pub use self::noisetexture::NoiseTexture;
pub use self::perlin::perlin_noise;

use crate::render::Color;
use crate::{Point2, Vector3};
use std::sync::Arc;

pub trait Texture: Sync {
    fn get_value(&self, uv_coords: Point2<f32>, p: Vector3<f32>) -> Color;
}

pub type ThreadTexture = Arc<Box<dyn Texture + Send>>;
