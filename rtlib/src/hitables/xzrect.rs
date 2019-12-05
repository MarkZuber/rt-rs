use crate::hitables::{HitRecord, Hitable, ThreadHitable};
use crate::next_rand_f32;
use crate::render::Ray;
use crate::{vec3, InnerSpace, Point2, Vector3};
use std::f32;
use std::sync::Arc;

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

impl Hitable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
            self.material_id,
            Point2::new(
                (x - self.x0) / (self.x1 - self.x0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
        ))
    }

    // fn get_bounding_box(&self, t0: f32, t1: f32) -> AABB {
    // return new AABB(new Vector3(X0, K - 0.001f, Z0), new Vector3(X1, K + 0.0001f, Z1));
    // }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        if let Some(hr) = self.hit(&Ray::new(origin, v), 0.001_f32, f32::MAX) {
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
