use crate::cameras::ThreadCamera;
use crate::next_rand_f32;
use crate::render::{
    Color, PixelBuffer, RayTracer, RenderConfig, Renderer, SamplingRayTracer, Scene,
};
use crate::stats::reset_stats;
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

    fn render_pixel(
        &self,
        pixel_buffer: &Arc<Mutex<PixelBuffer>>,
        the_scene: &Arc<Box<Scene>>,
        the_camera: &ThreadCamera,
        render_config: &RenderConfig,
        ray_tracer: &SamplingRayTracer,
        width_f32: f32,
        height_f32: f32,
        num_samples_f32: f32,
        x: u32,
        y: u32,
    ) {
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
            .de_nan()
            .multiply_by_scalar(1.0 / num_samples_f32)
            .apply_gamma();

        {
            let mut pixbuf = pixel_buffer.lock().unwrap();
            pixbuf.set_pixel_color(x, y, color)
        }
    }

    fn render_threaded(
        &self,
        pixel_buffer: &Arc<Mutex<PixelBuffer>>,
        the_scene: &Arc<Box<Scene>>,
        the_camera: &ThreadCamera,
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

        let half_height = image_height / 2;
        let half_width = image_width / 2;

        for y in 0..half_height {
            for x in 0..half_width {
                self.render_pixel(
                    &pixel_buffer,
                    &the_scene,
                    &the_camera,
                    &render_config,
                    &ray_tracer,
                    width_f32,
                    height_f32,
                    num_samples_f32,
                    half_width - x - 1,
                    half_height - y - 1,
                );
                self.render_pixel(
                    &pixel_buffer,
                    &the_scene,
                    &the_camera,
                    &render_config,
                    &ray_tracer,
                    width_f32,
                    height_f32,
                    num_samples_f32,
                    half_width - x - 1,
                    half_height + y,
                );
                self.render_pixel(
                    &pixel_buffer,
                    &the_scene,
                    &the_camera,
                    &render_config,
                    &ray_tracer,
                    width_f32,
                    height_f32,
                    num_samples_f32,
                    half_width + x,
                    half_height - y - 1,
                );
                self.render_pixel(
                    &pixel_buffer,
                    &the_scene,
                    &the_camera,
                    &render_config,
                    &ray_tracer,
                    width_f32,
                    height_f32,
                    num_samples_f32,
                    half_width + x,
                    half_height + y,
                );
            }

            (self.per_line_callback)(y);
        }
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
        // // Quick pre-render
        // self.render_threaded(
        //     &pixel_buffer,
        //     &the_scene,
        //     &the_camera,
        //     &RenderConfig::new(render_config.width, render_config.height, 5, 1),
        // );

        self.render_threaded(&pixel_buffer, &the_scene, &the_camera, render_config);
    }
}
