use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::pdfs::OrthoNormalBase;
use crate::random_to_sphere;
use crate::render::Ray;
use crate::{to_unit_vector, vec3, InnerSpace, Point2, Vector3};
use std::sync::Arc;
use std::{f32, fmt};

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    radius_sq: f32,
    material_id: u64,
    bounding_box: Arc<Box<AABB>>,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material_id: u64) -> ThreadHitable {
        let bounding_box = AABB::new(
            center - vec3(radius, radius, radius),
            center + vec3(radius, radius, radius),
        );

        Arc::new(Box::new(Sphere {
            center: center,
            radius: radius,
            radius_sq: radius * radius,
            material_id,
            bounding_box,
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
        let phi = punit.z.atan2(punit.x);
        let theta = punit.y.asin();
        let u = 1.0 - ((phi + pi) / (2.0 * pi));
        let v = (theta + (pi / 2.0)) / pi;
        Point2::new(u, v)
    }

    pub fn get_material_id(&self) -> u64 {
        self.material_id
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Sphere(center: {:?}, radius: {})]",
            self.center, self.radius
        )
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        info!("sphere::hit()");

        // if self.get_bounding_box(t_min, t_max).hit(ray, t_min, t_max) {
        let oc = ray.get_origin() - self.center();
        let a = ray.get_direction().dot(ray.get_direction());
        let b = oc.dot(ray.get_direction());
        let c = oc.dot(oc) - (self.radius_sq);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            let disc_sqrt = discriminant.sqrt();
            let t = (-b - disc_sqrt) / a;
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
            let t: f32 = (-b + disc_sqrt) / a;
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
        // }
        None
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        self.bounding_box.clone()
        // let radius = self.radius();
        // AABB::new(
        //     self.center - vec3(radius, radius, radius),
        //     self.center + vec3(radius, radius, radius),
        // )
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        match self.hit(&Ray::new(origin, v), 0.001, f32::MAX) {
            Some(_hr) => {
                let centoriginmag2 = (self.center() - origin).magnitude2();
                let cos_theta_max = (1.0 - (self.radius_sq / centoriginmag2)).sqrt();
                let solid_angle = 2.0 * f32::consts::PI * (1.0 - cos_theta_max);
                return 1.0 / solid_angle;
            }
            None => return 0.0,
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
