use crate::cameras::ThreadCamera;
use crate::next_rand_f32;
use crate::render::{
    Color, PixelBuffer, RayTracer, RenderConfig, Renderer, SamplingRayTracer, Scene,
};
use indicatif::{ProgressBar, ProgressStyle};
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
        the_camera: ThreadCamera,
        render_config: &RenderConfig,
    ) {
        let show_bar = render_config.show_progress_bar;

        let bar = ProgressBar::new(pixel_buffer.get_height() as u64);
        if show_bar {
            bar.set_style(ProgressStyle::default_bar().template(
            "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
        ));
        }

        let ray_tracer = SamplingRayTracer::new();

        let width_f32 = pixel_buffer.get_width() as f32;
        let height_f32 = pixel_buffer.get_height() as f32;
        let num_samples_f32 = render_config.num_samples as f32;

        for y in 0..pixel_buffer.get_height() {
            for x in 0..pixel_buffer.get_width() {
                let mut color: Color = (0..render_config.num_samples)
                    .into_par_iter()
                    // .into_iter()
                    .map(|_sample| {
                        info!("begin render pixel sample ({}, {})", x, y);
                        let scene = the_scene.clone();
                        let camera = the_camera.clone();

                        let u = (x as f32 + next_rand_f32()) / width_f32;
                        let v = (y as f32 + next_rand_f32()) / height_f32;

                        let ray = camera.get_ray(u, v);

                        let clr = ray_tracer.get_ray_color(&ray, scene, render_config, 0);
                        info!("end render pixel sample ({}, {})", x, y);

                        clr
                    })
                    .sum();

                color = color
                    .multiply_by_scalar(1.0 / num_samples_f32)
                    .apply_gamma();

                pixel_buffer.set_pixel_color(x, y, color)
            }
            if show_bar {
                bar.inc(1);
            }
        }

        if show_bar {
            bar.finish();
        }
    }
}
