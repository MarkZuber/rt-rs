use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::next_rand_f32;
use crate::render::Ray;
use crate::stats::RenderStats;
use crate::{vec3, InnerSpace, Point2, Vector3};
use std::sync::Arc;
use std::{f32, fmt};

pub struct XzRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material_id: u64,
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material_id: u64) -> ThreadHitable {
        Arc::new(Box::new(XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material_id,
        }))
    }
}

impl fmt::Display for XzRect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[XzRect(x0: {}, x1: {}, z0: {}, z1: {}, k: {})]",
            self.x0, self.x1, self.z0, self.z1, self.k
        )
    }
}

impl Hitable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, stat: &mut RenderStats) -> Option<HitRecord> {
        info!("xzrect::hit()");
        stat.xz_rect_hit();
        let t = (self.k - ray.get_origin().y) / ray.get_direction().y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.get_origin().x + (t * ray.get_direction().x);
        let z = ray.get_origin().z + (t * ray.get_direction().z);
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.get_point_at_parameter(t),
            Vector3::unit_y(),
            0.0, // todo: distance_squared
            self.material_id,
            Point2::new(
                (x - self.x0) / (self.x1 - self.x0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
        ))
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        AABB::new(
            vec3(self.x0, self.k - 0.001, self.z0),
            vec3(self.x1, self.k + 0.0001, self.z1),
        )
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>, stat: &mut RenderStats) -> f32 {
        if let Some(hr) = self.hit(&Ray::new(origin, v, stat), 0.001_f32, f32::MAX, stat) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = hr.t * hr.t * v.magnitude2();
            let cosine = (v.dot(hr.get_normal()) / v.magnitude()).abs();
            distance_squared / (cosine * area)
        } else {
            0.0_f32
        }
    }

    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        let random_point = vec3(
            self.x0 + (next_rand_f32() * (self.x1 - self.x0)),
            self.k,
            self.z0 + (next_rand_f32() * (self.z1 - self.z0)),
        );
        random_point - origin
    }
}
