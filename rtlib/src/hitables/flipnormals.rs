use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::render::Ray;
use crate::stats::RenderStats;
use crate::Vector3;
use std::fmt;
use std::sync::Arc;

pub struct FlipNormals {
    hitable: ThreadHitable,
}

impl FlipNormals {
    pub fn new(hitable: ThreadHitable) -> ThreadHitable {
        Arc::new(Box::new(FlipNormals { hitable }))
    }
}

impl fmt::Display for FlipNormals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[FlipNormals({})]", self.hitable)
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, stat: &mut RenderStats) -> Option<HitRecord> {
        info!("flipnormals::hit()");
        if let Some(hr) = self.hitable.hit(ray, t_min, t_max, stat) {
            return Some(HitRecord::new(
                hr.get_t(),
                hr.get_p(),
                -hr.get_normal(),
                hr.get_material_id(),
                hr.get_uv_coords(),
            ));
        }
        None
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>, stat: &mut RenderStats) -> f32 {
        self.hitable.get_pdf_value(origin, v, stat)
    }
    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        self.hitable.random(origin)
    }

    fn get_bounding_box(&self, t0: f32, t1: f32) -> Arc<Box<AABB>> {
        self.hitable.get_bounding_box(t0, t1)
    }
}
