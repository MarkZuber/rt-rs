use crate::cameras::ThreadCamera;
use crate::render::{PixelBuffer, RenderConfig, Scene};
use std::sync::{Arc, Mutex};

pub trait Renderer {
    fn render(
        &self,
        pixel_buffer: Arc<Mutex<PixelBuffer>>,
        the_scene: Arc<Box<Scene>>,
        the_camera: ThreadCamera,
        render_config: &RenderConfig,
    );
}
