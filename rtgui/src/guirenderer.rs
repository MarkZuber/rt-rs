use rtlib::cameras::ThreadCamera;
use rtlib::render::{ImagePixelBuffer, PerPixelRenderer, RenderConfig, Renderer, Scene};
use std::sync::{Arc, Mutex};

pub struct GuiRenderer {}

impl GuiRenderer {
    pub fn new() -> Box<dyn Renderer> {
        Box::new(GuiRenderer {})
    }
}

impl Renderer for GuiRenderer {
    fn render(
        &self,
        pixel_buffer: Arc<Mutex<ImagePixelBuffer>>,
        the_scene: Arc<Box<Scene>>,
        the_camera: ThreadCamera,
        render_config: &RenderConfig,
    ) {
        let rend = PerPixelRenderer::new(Arc::new(move |_yval| {}));
        rend.render(pixel_buffer, the_scene, the_camera, render_config);
    }
}
