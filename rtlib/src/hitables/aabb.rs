use crate::hitables::to_single_array;
use crate::render::Ray;
use crate::{vec3, Vector3};
use std::sync::Arc;

pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Arc<Box<AABB>> {
        Arc::new(Box::new(AABB { min, max }))
    }

    pub fn new_empty() -> Arc<Box<AABB>> {
        AABB::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0))
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let minvec = to_single_array(self.min);
        let maxvec = to_single_array(self.max);
        let originvec = to_single_array(ray.get_origin());
        let dirvec = to_single_array(ray.get_direction());

        for a in 0..3 {
            let inv_d = 1.0 / dirvec[a];
            let mut t0 = (minvec[a] - originvec[a]) * inv_d;
            let mut t1 = (maxvec[a] - originvec[a]) * inv_d;
            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }

            let new_t_min = t0.max(t_min);
            let new_t_max = t1.min(t_max);
            if new_t_max <= new_t_min {
                return false;
            }
        }

        true
    }

    pub fn get_surrounding_box(&self, other: Arc<Box<AABB>>) -> Arc<Box<AABB>> {
        let small = vec3(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );

        let big = vec3(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );

        AABB::new(small, big)
    }
}
