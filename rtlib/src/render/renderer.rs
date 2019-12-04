use crate::cameras::Camera;
use crate::render::{PixelBuffer, RenderConfig, Scene};
use std::sync::Arc;

pub trait Renderer {
    fn render(
        &self,
        pixel_buffer: &mut dyn PixelBuffer,
        the_scene: Arc<Box<Scene>>,
        camera: Box<dyn Camera>,
        render_config: &RenderConfig,
    );
}
