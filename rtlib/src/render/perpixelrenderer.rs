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
        the_scene: &Arc<Box<Scene>>,
        the_camera: &ThreadCamera,
        render_config: &RenderConfig,
        ray_tracer: &SamplingRayTracer,
        width_f32: f32,
        height_f32: f32,
        num_samples_f32: f32,
        x: u32,
        y: u32,
    ) -> ColorWithStat {
        let mut color_with_stat: ColorWithStat = (0..render_config.num_samples)
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

        color_with_stat.color = color_with_stat
            .color
            .de_nan()
            .multiply_by_scalar(1.0 / num_samples_f32)
            .apply_gamma();
        color_with_stat
    }

    fn render_line(
        &self,
        pixel_buffer: &Arc<Mutex<PixelBuffer>>,
        the_scene: &Arc<Box<Scene>>,
        the_camera: &ThreadCamera,
        render_config: &RenderConfig,
        ray_tracer: &SamplingRayTracer,
        width_f32: f32,
        height_f32: f32,
        num_samples_f32: f32,
        image_width: u32,
        y: u32,
    ) -> RenderStats {
        let mut render_stats = RenderStats::new();

        // collect pixels per line
        let mut line_colors: Vec<Color> = Vec::new();
        for x in 0..image_width {
            let stat_with_color = self.render_pixel(
                &the_scene,
                &the_camera,
                &render_config,
                &ray_tracer,
                width_f32,
                height_f32,
                num_samples_f32,
                x,
                y,
            );
            render_stats = render_stats.add(stat_with_color.stat);
            line_colors.push(stat_with_color.color);
        }
        {
            let mut pixbuf = pixel_buffer.lock().unwrap();
            for x in 0..image_width {
                pixbuf.set_pixel_color(x, y, line_colors[x as usize]);
            }
        }
        (self.per_line_callback)(y);

        render_stats
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

        // Switch between two different types of rendering to test lock contention on the pixel buffer.
        // Initial testing doesn't show any evidence of lock contention.
        // The only lock sharing is between the renderer (which is single threaded for accessing the buffer)
        // and the screen renderer (pulling contents of the buffer to draw while rendering).
        // I like the render_per_line code better since it's less verbose but it means i can only
        // render on a per-line basis where the other could do more progressive rendering if desired.
        let render_per_line = true;

        if render_per_line {
            for y in 0..half_height {
                {
                    let stats = self.render_line(
                        &pixel_buffer,
                        &the_scene,
                        &the_camera,
                        &render_config,
                        &ray_tracer,
                        width_f32,
                        height_f32,
                        num_samples_f32,
                        image_width,
                        half_height - y - 1,
                    );
                    render_stats = render_stats.add(stats);
                }
                {
                    let stats = self.render_line(
                        &pixel_buffer,
                        &the_scene,
                        &the_camera,
                        &render_config,
                        &ray_tracer,
                        width_f32,
                        height_f32,
                        num_samples_f32,
                        image_width,
                        half_height + y,
                    );
                    render_stats = render_stats.add(stats);
                }
            }
        } else {
            for y in 0..half_height {
                for x in 0..half_width {
                    {
                        let stat_with_color = self.render_pixel(
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
                        render_stats = render_stats.add(stat_with_color.stat);
                        {
                            let mut pixbuf = pixel_buffer.lock().unwrap();
                            pixbuf.set_pixel_color(
                                half_width - x - 1,
                                half_height - y - 1,
                                stat_with_color.color,
                            );
                        }
                    }
                    {
                        let stat_with_color = self.render_pixel(
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
                        render_stats = render_stats.add(stat_with_color.stat);
                        {
                            let mut pixbuf = pixel_buffer.lock().unwrap();
                            pixbuf.set_pixel_color(
                                half_width - x - 1,
                                half_height + y,
                                stat_with_color.color,
                            );
                        }
                    }
                    {
                        let stat_with_color = self.render_pixel(
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
                        render_stats = render_stats.add(stat_with_color.stat);
                        {
                            let mut pixbuf = pixel_buffer.lock().unwrap();
                            pixbuf.set_pixel_color(
                                half_width + x,
                                half_height - y - 1,
                                stat_with_color.color,
                            );
                        }
                    }
                    {
                        let stat_with_color = self.render_pixel(
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
                        render_stats = render_stats.add(stat_with_color.stat);
                        {
                            let mut pixbuf = pixel_buffer.lock().unwrap();
                            pixbuf.set_pixel_color(
                                half_width + x,
                                half_height + y,
                                stat_with_color.color,
                            );
                        }
                    }
                }

                (self.per_line_callback)(half_height - y - 1);
                (self.per_line_callback)(half_height + y);
            }
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
