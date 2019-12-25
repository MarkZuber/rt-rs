use crate::cameras::ThreadCamera;
use crate::render::{ImagePixelBuffer, PerPixelRenderer, RenderConfig, Renderer, Scene};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};

pub struct ConsoleRenderer {}

impl ConsoleRenderer {
    pub fn new() -> Box<dyn Renderer> {
        Box::new(ConsoleRenderer {})
    }
}

impl Renderer for ConsoleRenderer {
    fn render(
        &self,
        pixel_buffer: Arc<Mutex<ImagePixelBuffer>>,
        the_scene: Arc<Box<Scene>>,
        the_camera: ThreadCamera,
        render_config: &RenderConfig,
    ) {
        let show_bar = Arc::new(render_config.show_progress_bar);

        let image_height: u32;
        {
            let pixbuf = pixel_buffer.lock().unwrap();
            image_height = pixbuf.get_height();
        }

        let bar = Arc::new(ProgressBar::new(image_height as u64));
        if *show_bar {
            bar.set_style(ProgressStyle::default_bar().template(
            "[{elapsed} elapsed] {wide_bar:.cyan/white} {percent}% [{eta} remaining] [rendering]",
        ));
        }

        let sb = show_bar.clone();
        let bc = bar.clone();
        let rend = PerPixelRenderer::new(Arc::new(move |_yval| {
            if *sb {
                bc.inc(1);
            }
        }));

        rend.render(pixel_buffer, the_scene, the_camera, render_config);

        if *show_bar {
            bar.finish();
        }
    }
}
