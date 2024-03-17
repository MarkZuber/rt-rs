use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::Sender,
    Arc,
};

use rtlib::{
    cameras::Camera,
    next_rand_f32,
    render::{Color, Pixel, RayTracer, RenderConfig, SamplingRayTracer, Scene, SceneGenerator},
    stats::RenderStats,
};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

struct Inner {
    width: u32,
    height: u32,
    pixel_tx: Sender<Pixel>,
    stop_requested: Arc<AtomicBool>, // arc so we can clone it into the subthreads doing the rendering
}

pub struct ImguiRenderer {
    inner: Arc<Inner>,
}

impl ImguiRenderer {
    pub fn new(width: u32, height: u32, pixel_tx: Sender<Pixel>) -> Self {
        let inner = Inner {
            width,
            height,
            pixel_tx,
            stop_requested: Arc::new(AtomicBool::new(false)),
        };

        ImguiRenderer {
            inner: Arc::new(inner),
        }
    }

    pub fn start(
        &self,
        render_config: Arc<RenderConfig>,
        scene_generator: Arc<Box<dyn SceneGenerator + Send>>,
    ) {
        let num_threads = 16;
        let segment_size = self.inner.height / num_threads;

        let the_scene = Arc::new(scene_generator.get_scene());
        let the_camera = scene_generator.get_camera();

        for sub_num in 0..num_threads {
            let local_self = self.inner.clone();
            let y_start = sub_num * segment_size;
            let rendconf = render_config.clone();
            let scene = the_scene.clone();
            let cam = the_camera.clone();
            std::thread::spawn(move || {
                local_self.do_render(y_start, y_start + segment_size, rendconf, scene, cam);
            });
        }
    }

    pub fn start_ex(
        &self,
        render_config: Arc<RenderConfig>,
        scene_generator: Arc<Box<dyn SceneGenerator + Send>>,
    ) {
        let local_self = self.inner.clone();

        std::thread::spawn(move || {
            local_self.do_render_ex(render_config, scene_generator);
        });
    }

    pub fn stop(&self) {
        self.inner.stop_requested.store(true, Ordering::Relaxed);
    }
}

impl Inner {
    fn do_render_ex(
        &self,
        render_config: Arc<RenderConfig>,
        scene_generator: Arc<Box<dyn SceneGenerator + Send>>,
    ) {
        let width_f32 = self.width as f32;
        let height_f32 = self.height as f32;
        let ray_tracer = SamplingRayTracer::new();
        let num_samples_f32 = render_config.num_samples as f32;
        let the_camera = scene_generator.get_camera();
        let the_scene = scene_generator.get_scene();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut color: Color = (0..render_config.num_samples)
                    .into_par_iter()
                    .map(|_sample| {
                        let mut stat = RenderStats::new();
                        let u = (x as f32 + next_rand_f32()) / width_f32;
                        let v = (y as f32 + next_rand_f32()) / height_f32;
                        let ray = the_camera.get_ray(u, v, &mut stat);
                        let clr = ray_tracer.get_ray_color(
                            &mut stat,
                            &ray,
                            &the_scene,
                            &render_config,
                            0,
                        );
                        clr
                    })
                    .sum();

                color = color
                    .de_nan()
                    .multiply_by_scalar(1.0 / num_samples_f32)
                    .apply_gamma();

                self.pixel_tx.send(Pixel::new(x, y, color)).unwrap();
            }
        }
    }

    fn do_render(
        &self,
        y_start: u32,
        y_end: u32,
        render_config: Arc<RenderConfig>,
        the_scene: Arc<Scene>,
        the_camera: Arc<Box<dyn Camera + Send>>,
    ) {
        let width_f32 = self.width as f32;
        let height_f32 = self.height as f32;
        let ray_tracer = SamplingRayTracer::new();
        let num_samples_f32 = render_config.num_samples as f32;

        // TODO: when we mulththread the renderer, we will need to clone the sender
        for y in y_start..y_end {
            for x in 0..self.width {
                if self.stop_requested.load(Ordering::Relaxed) {
                    return;
                }

                let mut final_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..render_config.num_samples {
                    let mut stat = RenderStats::new();
                    let u = (x as f32 + next_rand_f32()) / width_f32;
                    let v = (y as f32 + next_rand_f32()) / height_f32;
                    let ray = the_camera.get_ray(u, v, &mut stat);
                    let clr =
                        ray_tracer.get_ray_color(&mut stat, &ray, &the_scene, &render_config, 0);
                    final_color = final_color.add(clr);
                }

                final_color = final_color
                    .de_nan()
                    .multiply_by_scalar(1.0 / num_samples_f32)
                    .apply_gamma(); // todo: adopt anyhow here for error handling

                self.pixel_tx.send(Pixel::new(x, y, final_color)).unwrap();

                /*
                if self.is_red {
                    self.pixel_tx
                        .send(Pixel::new(
                            x,
                            y,
                            Color::new(
                                y_end as f32 / self.height as f32,
                                0.0,
                                x as f32 / self.width as f32,
                            ),
                        ))
                        .unwrap();
                } else {
                    self.pixel_tx
                        .send(Pixel::new(x, y, Color::new(0.0, 0.0, 1.0)))
                        .unwrap();
                }
                */
            }
        }
    }
}
