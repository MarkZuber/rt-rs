use crate::cameras::ThreadCamera;
use crate::next_rand_f32;
use crate::render::{
    Color, PixelBuffer, RayTracer, RenderConfig, Renderer, SamplingRayTracer, Scene,
};
use crate::stats::RenderStats;
use core::iter::Sum;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};

pub type PerLineCallbackFunc = Arc<(dyn Fn(u32) + Send + Sync)>;

pub struct PerPixelRenderer {
    per_line_callback: PerLineCallbackFunc,
}

struct ColorWithStat {
    color: Color,
    stat: RenderStats,
}

impl Sum for ColorWithStat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            Self {
                color: Color::zero(),
                stat: RenderStats::default(),
            },
            |a, b| Self {
                color: a.color.add(b.color),
                stat: a.stat.add(b.stat),
            },
        )
    }
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
    ) -> RenderStats {
        let color_with_stat: ColorWithStat = (0..render_config.num_samples)
            .into_par_iter()
            .map(|_sample| {
                let mut stat = RenderStats::new();
                let u = (x as f32 + next_rand_f32()) / width_f32;
                let v = (y as f32 + next_rand_f32()) / height_f32;
                let ray = the_camera.get_ray(u, v, &mut stat);
                let clr = ray_tracer.get_ray_color(&mut stat, &ray, &the_scene, render_config, 0);
                ColorWithStat { color: clr, stat }
            })
            .sum();

        let mut color = color_with_stat.color;

        color = color
            .de_nan()
            .multiply_by_scalar(1.0 / num_samples_f32)
            .apply_gamma();

        {
            let mut pixbuf = pixel_buffer.lock().unwrap();
            pixbuf.set_pixel_color(x, y, color);
        }

        color_with_stat.stat
    }

    fn render_threaded(
        &self,
        pixel_buffer: &Arc<Mutex<PixelBuffer>>,
        the_scene: &Arc<Box<Scene>>,
        the_camera: &ThreadCamera,
        render_config: &RenderConfig,
    ) -> RenderStats {
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

        let mut render_stats = RenderStats::new();

        for y in 0..half_height {
            for x in 0..half_width {
                render_stats = render_stats.add(self.render_pixel(
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
                ));
                render_stats = render_stats.add(self.render_pixel(
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
                ));
                render_stats = render_stats.add(self.render_pixel(
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
                ));
                render_stats = render_stats.add(self.render_pixel(
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
                ));
            }

            (self.per_line_callback)(half_height - y - 1);
            (self.per_line_callback)(half_height + y);
        }

        render_stats
    }
}

impl Renderer for PerPixelRenderer {
    fn render(
        &self,
        pixel_buffer: Arc<Mutex<PixelBuffer>>,
        the_scene: Arc<Box<Scene>>,
        the_camera: ThreadCamera,
        render_config: &RenderConfig,
    ) -> RenderStats {
        // // Quick pre-render
        self.render_threaded(
            &pixel_buffer,
            &the_scene,
            &the_camera,
            &RenderConfig::new(render_config.width, render_config.height, 5, 1),
        );

        self.render_threaded(&pixel_buffer, &the_scene, &the_camera, render_config)
    }
}
