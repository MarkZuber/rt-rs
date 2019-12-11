mod camera;
mod normalcamera;
use std::sync::Arc;

pub use self::camera::Camera;
pub use self::normalcamera::NormalCamera;

pub type ThreadCamera = Arc<Box<dyn Camera + Send>>;
