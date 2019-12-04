use crate::render::Ray;

pub trait Camera: Sync {
    fn get_ray(&self, s: f32, t: f32) -> Ray;
}
