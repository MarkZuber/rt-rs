use crate::cameras::ThreadCamera;
use crate::next_rand_f32;
use crate::render::{
    Color, PixelBuffer, RayTracer, RenderConfig, Renderer, SamplingRayTracer, Scene,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};

pub type PerLineCallbackFunc = Arc<(dyn Fn(u32) + Send + Sync)>;

pub struct PerPixelRenderer {
    per_line_callback: PerLineCallbackFunc,
}

impl PerPixelRenderer {
    pub fn new(per_line_callback: PerLineCallbackFunc) -> impl Renderer {
        PerPixelRenderer { per_line_callback }
    }
}

impl Renderer for PerPixelRenderer {
    fn render(
        &self,
        pixel_buffer: Arc<Mutex<PixelBuffer>>,
        the_scene: Arc<Box<Scene>>,
        the_camera: ThreadCamera,
        render_config: &RenderConfig,
    ) {
        let ray_tracer = SamplingRayTracer::new();

        let image_width: u32;
        let image_height: u32;

        {
            let pixbuf = pixel_buffer.lock().unwrap();
            image_width = pixbuf.get_width();
            image_height = pixbuf.get_height();
        }

        let width_f32 = image_width as f32;
        let height_f32 = image_height as f32;
        let num_samples_f32 = render_config.num_samples as f32;

        for y in 0..image_height {
            for x in 0..image_width {
                let mut color: Color = (0..render_config.num_samples)
                    .into_par_iter()
                    .map(|_sample| {
                        info!("begin render pixel sample ({}, {})", x, y);
                        let scene = the_scene.clone();
                        let camera = the_camera.clone();

                        let u = (x as f32 + next_rand_f32()) / width_f32;
                        let v = (y as f32 + next_rand_f32()) / height_f32;

                        let ray = camera.get_ray(u, v);

                        let clr = ray_tracer.get_ray_color(&ray, &scene, render_config, 0);
                        info!("end render pixel sample ({}, {})", x, y);

                        clr
                    })
                    .sum();

                color = color
                    .multiply_by_scalar(1.0 / num_samples_f32)
                    .apply_gamma();

                {
                    let mut pixbuf = pixel_buffer.lock().unwrap();
                    pixbuf.set_pixel_color(x, y, color)
                }
            }

            (self.per_line_callback)(y);
        }
    }
}
