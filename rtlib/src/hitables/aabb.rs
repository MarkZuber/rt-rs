use crate::render::Ray;
use crate::stats::{record_stat, RenderStat};
use crate::{vec3, Vector3};
use std::fmt;
use std::sync::Arc;

pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next_rand_f32;
    use test::Bencher;

    #[bench]
    fn bench_render(b: &mut Bencher) {
        let ab = AABB::new(
            vec3(next_rand_f32(), next_rand_f32(), next_rand_f32()),
            vec3(next_rand_f32(), next_rand_f32(), next_rand_f32()),
        );
        b.iter(|| {
            ab.hit(
                &Ray::new(
                    vec3(next_rand_f32(), next_rand_f32(), next_rand_f32()),
                    vec3(next_rand_f32(), next_rand_f32(), next_rand_f32()),
                ),
                0.0,
                1.0,
            );
        });
    }
}

#[inline]
fn swap_if_first_greater(a: f32, b: f32) -> (f32, f32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Arc<Box<AABB>> {
        Arc::new(Box::new(AABB { min, max }))
    }

    pub fn new_empty() -> Arc<Box<AABB>> {
        AABB::new(vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0))
    }

    pub fn hit(&self, ray: &Ray, _t_min: f32, _t_max: f32) -> bool {
        info!("aabb::hit()");
        record_stat(RenderStat::AabbHit);

        let (mut txmin, mut txmax) = swap_if_first_greater(
            (self.min.x - ray.get_origin().x) / ray.get_direction().x,
            (self.max.x - ray.get_origin().x) / ray.get_direction().x,
        );

        let (tymin, tymax) = swap_if_first_greater(
            (self.min.y - ray.get_origin().y) / ray.get_direction().y,
            (self.max.y - ray.get_origin().y) / ray.get_direction().y,
        );

        if txmin > tymax || tymin > txmax {
            return false;
        }

        if tymin > txmin {
            txmin = tymin;
        }

        if tymax < txmax {
            txmax = tymax;
        }

        let (tzmin, tzmax) = swap_if_first_greater(
            (self.min.z - ray.get_origin().z) / ray.get_direction().z,
            (self.max.z - ray.get_origin().z) / ray.get_direction().z,
        );

        if txmin > tzmax || tzmin > txmax {
            return false;
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
