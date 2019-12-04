mod hitablelist;
mod hitrecord;
mod sphere;

pub use self::hitablelist::HitableList;
pub use self::hitrecord::HitRecord;
pub use self::sphere::Sphere;

use crate::render::Ray;
use crate::Vector3;

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    // fn get_bounding_box(&self, t0: f32, t1: f32) -> AABB;
    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32;
    fn random(&self, origin: Vector3<f32>) -> Vector3<f32>;
}
