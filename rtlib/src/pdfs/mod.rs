mod cosinepdf;
mod hitablepdf;
mod mixturepdf;
mod orthonormalbase;

pub use self::cosinepdf::CosinePdf;
pub use self::hitablepdf::HitablePdf;
pub use self::mixturepdf::MixturePdf;
pub use self::orthonormalbase::OrthoNormalBase;

use crate::Vector3;

pub trait Pdf {
    fn get_value(&self, direction: Vector3<f32>) -> f32;
    fn generate(&self) -> Vector3<f32>;
}
