use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::render::Ray;
use crate::stats::RenderStats;
use crate::Vector3;
use std::fmt;
use std::sync::Arc;

pub struct Translate {
    hitable: ThreadHitable,
    displacement: Vector3<f32>,
}

impl Translate {
    pub fn new(hitable: ThreadHitable, displacement: Vector3<f32>) -> ThreadHitable {
        Arc::new(Box::new(Translate {
            hitable,
            displacement,
        }))
    }
}

impl fmt::Display for Translate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Translate({}, displacement: {:?})]",
            self.hitable, self.displacement
        )
    }
}

impl Hitable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, stat: &mut RenderStats) -> Option<HitRecord> {
        info!("translate::hit()");
        let moved_ray = Ray::new(
            ray.get_origin() - self.displacement,
            ray.get_direction(),
            stat,
        );
        if let Some(hr) = self.hitable.hit(&moved_ray, t_min, t_max, stat) {
            return Some(HitRecord::new(
                hr.get_t(),
                hr.get_p() + self.displacement,
                hr.get_normal(),
                0.0, // todo: distance_squared
                hr.get_material_id(),
                hr.get_uv_coords(),
            ));
        }

        None
    }

    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        self.hitable.random(origin)
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>, stat: &mut RenderStats) -> f32 {
        self.hitable.get_pdf_value(origin, v, stat)
    }
    fn get_bounding_box(&self, t0: f32, t1: f32) -> Arc<Box<AABB>> {
        let b = self.hitable.get_bounding_box(t0, t1);
        AABB::new(b.min + self.displacement, b.max + self.displacement)
    }
}
