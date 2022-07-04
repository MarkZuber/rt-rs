use crate::{render::Ray, stats::RenderStats};

pub trait Camera: Sync {
    fn get_ray(&self, s: f32, t: f32, stat: &mut RenderStats) -> Ray;
}
