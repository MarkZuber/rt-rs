mod aabb;
mod bvhnode;
mod cube;
mod cylinder;
mod flipnormals;
mod hitablelist;
mod hitrecord;
mod medium;
mod rotatex;
mod rotatey;
mod rotatez;
mod sphere;
mod translate;
mod triangle;
mod xyrect;
mod xzrect;
mod yzrect;

pub use self::aabb::AABB;
pub use self::bvhnode::BvhNode;
pub use self::cube::Cube;
pub use self::cylinder::Cylinder;
pub use self::flipnormals::FlipNormals;
pub use self::hitablelist::HitableList;
pub use self::hitrecord::HitRecord;
pub use self::medium::ConstantMedium;
pub use self::rotatex::RotateX;
pub use self::rotatey::RotateY;
pub use self::rotatez::RotateZ;
pub use self::sphere::Sphere;
pub use self::translate::Translate;
pub use self::triangle::Triangle;
pub use self::xyrect::XyRect;
pub use self::xzrect::XzRect;
pub use self::yzrect::YzRect;

use crate::render::Ray;
use crate::stats::RenderStats;
use crate::Vector3;
use std::fmt;
use std::sync::Arc;

pub type ThreadHitable = Arc<Box<dyn Hitable + Send>>;

pub trait Hitable: Sync + fmt::Display {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, stat: &mut RenderStats) -> Option<HitRecord>;
    fn get_bounding_box(&self, t0: f32, t1: f32) -> Arc<Box<AABB>>;
    fn get_pdf_value(&self, _origin: Vector3<f32>, _v: Vector3<f32>, stat: &mut RenderStats)
        -> f32;
    fn random(&self, _origin: Vector3<f32>) -> Vector3<f32>;
}

fn to_single_array(v: Vector3<f32>) -> Vec<f32> {
    vec![v.x, v.y, v.z]
}
