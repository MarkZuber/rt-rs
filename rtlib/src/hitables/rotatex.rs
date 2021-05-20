use crate::hitables::{to_single_array, HitRecord, Hitable, ThreadHitable, AABB};
use crate::render::Ray;
use crate::{vec3, Vector3};
use std::sync::Arc;
use std::{f32, fmt};

pub struct RotateX {
    hitable: ThreadHitable,
    sin_theta: f32,
    cos_theta: f32,
    bounding_box: Arc<Box<AABB>>,
}

impl RotateX {
    pub fn new(hitable: ThreadHitable, angle: f32) -> ThreadHitable {
        let radians = (f32::consts::PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let b = hitable.get_bounding_box(0.0, 1.0);
        let mut min = to_single_array(vec3(f32::MAX, f32::MAX, f32::MAX));
        let mut max = to_single_array(vec3(-f32::MAX, -f32::MAX, -f32::MAX));

        for i in 0..2 {
            let dubi = i as f32;
            for j in 0..2 {
                let dubj = j as f32;
                for k in 0..2 {
                    let dubk = k as f32;
                    let x = (dubi * b.max.x) + ((1.0 - dubi) * b.min.x);
                    let y = (dubj * b.max.y) + ((1.0 - dubj) * b.min.y);
                    let z = (dubk * b.max.z) + ((1.0 - dubk) * b.min.z);
                    let newy = (cos_theta * y) - (sin_theta * z);
                    let newz = (sin_theta * y) + (cos_theta * z);
                    let tester = to_single_array(vec3(x, newy, newz));
                    for c in 0..3 {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }
                        if tester[c] < min[c] {
                            min[c] = tester[c]
                        }
                    }
                }
            }
        }

        let bounding_box = AABB::new(vec3(min[0], min[1], min[2]), vec3(max[0], max[1], max[2]));

        Arc::new(Box::new(RotateX {
            hitable,
            sin_theta,
            cos_theta,
            bounding_box,
        }))
    }
}

impl fmt::Display for RotateX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[RotateX({})]", self.hitable)
    }
}

impl Hitable for RotateX {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        info!("rotatex::hit()");
        let mut origin = to_single_array(ray.get_origin());
        let mut dir = to_single_array(ray.get_direction());
        origin[1] = (self.cos_theta * ray.get_origin().y) - (self.sin_theta * ray.get_origin().z);
        origin[2] = (self.sin_theta * ray.get_origin().y) + (self.cos_theta * ray.get_origin().z);
        dir[1] =
            (self.cos_theta * ray.get_direction().y) - (self.sin_theta * ray.get_direction().z);
        dir[2] =
            (self.sin_theta * ray.get_direction().y) + (self.cos_theta * ray.get_direction().z);
        let rotated_ray = Ray::new(
            vec3(origin[0], origin[1], origin[2]),
            vec3(dir[0], dir[1], dir[2]),
        );
        if let Some(hit_record) = self.hitable.hit(&rotated_ray, t_min, t_max) {
            let mut p = to_single_array(hit_record.get_p());
            let mut normal = to_single_array(hit_record.get_normal());
            p[1] =
                (self.cos_theta * hit_record.get_p().y) + (self.sin_theta * hit_record.get_p().z);
            p[2] =
                (-self.sin_theta * hit_record.get_p().y) + (self.cos_theta * hit_record.get_p().z);
            normal[1] = (self.cos_theta * hit_record.get_normal().y)
                + (self.sin_theta * hit_record.get_normal().z);
            normal[2] = (-self.sin_theta * hit_record.get_normal().y)
                + (self.cos_theta * hit_record.get_normal().z);

            return Some(HitRecord::new(
                hit_record.get_t(),
                vec3(p[0], p[1], p[2]),
                vec3(normal[0], normal[1], normal[2]),
                hit_record.get_material_id(),
                hit_record.get_uv_coords(),
            ));
        }

        None
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        self.hitable.get_pdf_value(origin, v)
    }
    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        self.hitable.random(origin)
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        self.bounding_box.clone()
    }
}
