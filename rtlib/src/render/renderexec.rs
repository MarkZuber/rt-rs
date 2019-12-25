use crate::cameras::ThreadCamera;
use crate::render::{PixelBuffer, RenderConfig, Renderer, Scene, SceneGenerator};
use std::sync::{Arc, Mutex};

pub struct RenderExec {
    pub pixel_buffer: Arc<Mutex<PixelBuffer>>,
    scene: Arc<Box<Scene>>,
    camera: ThreadCamera,
    render_config: RenderConfig,
    renderer: Box<dyn Renderer>,
}

impl RenderExec {
    pub fn new(
        scene_generator: Box<dyn SceneGenerator>,
        renderer: Box<dyn Renderer>,
        image_width: u32,
        image_height: u32,
        ray_trace_depth: u32,
        num_samples: u32,
        show_progress_bar: bool,
    ) -> RenderExec {
        let pixel_buffer = Arc::new(Mutex::new(PixelBuffer::new(image_width, image_height)));
        let render_config = RenderConfig::new(ray_trace_depth, num_samples, show_progress_bar);

        let scene = scene_generator.create_scene();
        let camera = scene_generator.create_camera(image_width, image_height);
        RenderExec {
            pixel_buffer,
            scene: Arc::new(Box::new(scene)),
            camera,
            render_config,
            renderer,
        }
    }

    pub fn execute(&mut self) {
        self.renderer.render(
            self.pixel_buffer.clone(),
            self.scene.clone(),
            self.camera.clone(),
            &self.render_config,
        );
    }

    pub fn save_pixel_buffer(&self, file_path: &str) {
        let pixbuf = self.pixel_buffer.lock().unwrap();
        pixbuf.save_as_png(file_path);
    }
}
