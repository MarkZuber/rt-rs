use crate::hitables::{HitRecord, Hitable, ThreadHitable};
use crate::pdfs::OrthoNormalBase;
use crate::random_to_sphere;
use crate::render::Ray;
use crate::{to_unit_vector, InnerSpace, Point2, Vector3};
use std::f32;
use std::sync::Arc;

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material_id: u64,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material_id: u64) -> ThreadHitable {
        Arc::new(Box::new(Sphere {
            center: center,
            radius: radius,
            material_id,
        }))
    }

    pub fn center(&self) -> Vector3<f32> {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn get_sphere_uv(&self, p: Vector3<f32>) -> Point2<f32> {
        let pi = std::f32::consts::PI;
        let punit = to_unit_vector(p);
        let phi: f32 = punit.z.atan2(punit.x);
        let theta: f32 = (punit.y).asin();
        let u: f32 = 1.0 - ((phi + pi) / (2.0 * pi));
        let v: f32 = (theta + (pi / 2.0)) / pi;
        Point2::new(u, v)
    }

    pub fn get_material_id(&self) -> u64 {
        self.material_id
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vector3<f32> = ray.get_origin() - self.center();
        let a: f32 = ray.get_direction().dot(ray.get_direction());
        let b: f32 = 2.0 * oc.dot(ray.get_direction());
        let c: f32 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t: f32 = (0.0 - b - discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let p = ray.get_point_at_parameter(t);
                // The length p - c would be the radius
                let normal = (p - self.center()) / self.radius();

                return Some(HitRecord::new(
                    t,
                    p,
                    normal,
                    self.get_material_id(),
                    self.get_sphere_uv(p),
                ));
            }
            let t: f32 = (0.0 - b + discriminant.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                let p = ray.get_point_at_parameter(t);
                // The length p - c would be the radius
                let normal = (p - self.center()) / self.radius();

                return Some(HitRecord::new(
                    t,
                    p,
                    normal,
                    self.get_material_id(),
                    self.get_sphere_uv(p),
                ));
            }
        }
        None
    }

    // fn get_bounding_box(&self, t0: f32, t1: f32) -> AABB {
    //     // return new AABB(Center - new Vector3(Radius, Radius, Radius), Center + new Vector3(Radius, Radius, Radius));
    //     AABB {}
    // }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        match self.hit(&Ray::new(origin, v), 0.001_f32, f32::MAX) {
            Some(_hr) => {
                let cos_theta_max = (1.0_f32
                    - (self.radius() * self.radius() / (self.center() - origin).magnitude2()))
                .sqrt();
                let solid_angle = 2.0_f32 * f32::consts::PI * (1.0_f32 - cos_theta_max);
                return 1.0_f32 / solid_angle;
            }
            None => return 0.0_f32,
        }
    }

    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        let direction = self.center() - origin;
        let distance_squared = direction.magnitude2();
        let uvw = OrthoNormalBase::from_w(direction);
        let v = random_to_sphere(self.radius(), distance_squared);
        return uvw.local(v);
    }
}
