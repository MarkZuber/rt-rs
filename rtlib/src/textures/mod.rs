mod colortexture;

pub use self::colortexture::ColorTexture;

use crate::render::Color;
use crate::{Point2, Vector3};

pub trait Texture: Sync {
    fn get_value(&self, uv_coords: Point2<f32>, p: Vector3<f32>) -> Color;
}
