use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::next_rand_f32;
use crate::render::Ray;
use crate::{vec3, InnerSpace, Point2, Vector3};
use std::sync::Arc;
use std::{f32, fmt};

pub struct YzRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material_id: u64,
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material_id: u64) -> ThreadHitable {
        Arc::new(Box::new(YzRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material_id,
        }))
    }
}

impl fmt::Display for YzRect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[YzRect(y0: {}, y1: {}, z0: {}, z1: {}, k: {})]",
            self.y0, self.y1, self.z0, self.z1, self.k
        )
    }
}

impl Hitable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        info!("yzrect::hit()");
        let t = (self.k - ray.get_origin().x) / ray.get_direction().x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.get_origin().y + (t * ray.get_direction().y);
        let z = ray.get_origin().z + (t * ray.get_direction().z);
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            ray.get_point_at_parameter(t),
            Vector3::unit_x(),
            self.material_id,
            Point2::new(
                (y - self.y0) / (self.y1 - self.y0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
        ))
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        AABB::new(
            vec3(self.k - 0.001, self.y0, self.z0),
            vec3(self.k + 0.0001, self.y1, self.z1),
        )
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        if let Some(hr) = self.hit(&Ray::new(origin, v), 0.001_f32, f32::MAX) {
            let area = (self.y1 - self.y0) * (self.z1 - self.z0);
            let distance_squared = hr.t * hr.t * v.magnitude2();
            let cosine = (v.dot(hr.get_normal()) / v.magnitude()).abs();
            distance_squared / (cosine * area)
        } else {
            0.0_f32
        }
    }

    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        let random_point = vec3(
            self.k,
            self.y0 + (next_rand_f32() * (self.y1 - self.y0)),
            self.z0 + (next_rand_f32() * (self.z1 - self.z0)),
        );
        random_point - origin
    }
}
