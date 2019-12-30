use crate::render::Color;
use crate::textures::{Texture, ThreadTexture};
use crate::{Point2, Vector3};
use std::sync::Arc;

pub struct CheckerTexture {
    t1: ThreadTexture,
    t2: ThreadTexture,
    scale: Vector3<f32>,
}

impl CheckerTexture {
    pub fn new(t1: ThreadTexture, t2: ThreadTexture, scale: Vector3<f32>) -> ThreadTexture {
        Arc::new(Box::new(CheckerTexture { t1, t2, scale }))
    }
}

impl Texture for CheckerTexture {
    fn get_value(&self, uv_coords: Point2<f32>, p: Vector3<f32>) -> Color {
        let sines =
            (self.scale.x * p.x).sin() * (self.scale.y * p.y).sin() * (self.scale.z * p.z).sin();
        if sines < 0.0 {
            self.t1.get_value(uv_coords, p)
        } else {
            self.t2.get_value(uv_coords, p)
        }
    }
}
