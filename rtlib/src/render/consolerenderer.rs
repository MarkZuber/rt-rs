use crate::cameras::ThreadCamera;
use crate::render::{PerPixelRenderer, PixelBuffer, RenderConfig, Renderer, Scene};
use crate::stats::RenderStats;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};

pub struct ConsoleRenderer {
    show_progress_bar: bool,
}

impl ConsoleRenderer {
    pub fn new(show_progress_bar: bool) -> Box<dyn Renderer> {
        Box::new(ConsoleRenderer { show_progress_bar })
    }
}

impl Renderer for ConsoleRenderer {
    fn render(
        &self,
        pixel_buffer: Arc<Mutex<PixelBuffer>>,
        the_scene: Arc<Box<Scene>>,
        the_camera: ThreadCamera,
        render_config: &RenderConfig,
    ) -> RenderStats {
        let show_bar = Arc::new(self.show_progress_bar);

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

        let stat = rend.render(pixel_buffer, the_scene, the_camera, render_config);

        if *show_bar {
            bar.finish();
        }
        stat
    }
}
