use crate::{
    render::{Color, Ray, RenderConfig, Scene},
    stats::RenderStats,
};

/// A RayTracer will bounce a ray into a scene and get its color.
pub trait RayTracer {
    /// Get the color of the ray bounced into the scene.
    fn get_ray_color(
        &self,
        stat: &mut RenderStats,
        ray: &Ray,
        the_scene: &Scene,
        render_config: &RenderConfig,
        depth: u32,
    ) -> Color;
}
