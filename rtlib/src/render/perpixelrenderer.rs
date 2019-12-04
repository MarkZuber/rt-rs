use crate::cameras::Camera;
use crate::render::{
    Color, PixelBuffer, RayTracer, RenderConfig, Renderer, SamplingRayTracer, Scene,
};
use indicatif::{ProgressBar, ProgressStyle};
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;

pub struct PerPixelRenderer {}

impl PerPixelRenderer {
    pub fn new() -> Box<dyn Renderer> {
        Box::new(PerPixelRenderer {})
    }
}

impl Renderer for PerPixelRenderer {
    fn render(
        &self,
        pixel_buffer: &mut dyn PixelBuffer,
        the_scene: Arc<Box<Scene>>,
        camera: Box<dyn Camera>,
        render_config: &RenderConfig,
    ) {
        let bar = ProgressBar::new(pixel_buffer.get_height() as u64);
        bar.set_style(ProgressStyle::default_bar().template(
            "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
        ));

        let ray_tracer = SamplingRayTracer::new();

        for y in 0..pixel_buffer.get_height() {
            for x in 0..pixel_buffer.get_width() {
                let mut color: Color = (0..render_config.num_samples)
                    .into_par_iter()
                    .map(|_sample| {
                        let mut rng = thread_rng();
                        let scene = the_scene.clone();

                        let u = (x as f32 + rng.gen::<f32>()) / pixel_buffer.get_width() as f32;
                        let v = (y as f32 + rng.gen::<f32>()) / pixel_buffer.get_height() as f32;

                        let ray = camera.get_ray(u, v);

                        ray_tracer
                            .get_ray_color(&ray, scene, render_config, 0)
                            .apply_gamma()

                        // ColorVector::new(
                        //     (x as f32) / (pixel_buffer.get_width() as f32),
                        //     (y as f32) / (pixel_buffer.get_height() as f32),
                        //     0.7)
                    })
                    .sum();

                color = color.multiply_by_scalar(1.0 / render_config.num_samples as f32);

                pixel_buffer.set_pixel_color(x, y, color)
            }
            bar.inc(1);
        }

        bar.finish();
    }
}
