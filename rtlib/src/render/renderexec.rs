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
        scene_generator: Arc<Box<dyn SceneGenerator + Send>>,
        renderer: Box<dyn Renderer>,
    ) -> RenderExec {
        let render_config = scene_generator.get_render_config();
        let pixel_buffer = Arc::new(Mutex::new(PixelBuffer::new(
            render_config.width,
            render_config.height,
        )));

        let scene = scene_generator.get_scene();
        let camera = scene_generator.get_camera();
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
