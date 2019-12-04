use crate::render::{Color, Ray, RenderConfig, Scene};
use std::sync::Arc;

pub trait RayTracer {
    fn get_ray_color(
        &self,
        ray: &Ray,
        the_scene: Arc<Box<Scene>>,
        render_config: &RenderConfig,
        depth: u32,
    ) -> Color;
}
