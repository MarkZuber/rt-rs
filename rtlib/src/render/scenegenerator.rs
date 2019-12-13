use crate::cameras::ThreadCamera;
use crate::render::Scene;

pub trait SceneGenerator {
    fn create_scene(&self) -> Scene;
    fn create_camera(&self, image_width: u32, image_height: u32) -> ThreadCamera;
}
