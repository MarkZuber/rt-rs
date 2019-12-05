use crate::hitables::{HitRecord, Hitable, ThreadHitable};
use crate::render::Ray;
use std::sync::Arc;

pub struct FlipNormals {
    hitable: ThreadHitable,
}

impl FlipNormals {
    pub fn new(hitable: ThreadHitable) -> ThreadHitable {
        Arc::new(Box::new(FlipNormals { hitable }))
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(hr) = self.hitable.hit(ray, t_min, t_max) {
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

    // fn get_bounding_box(&self, t0: f32, t1: f32) -> AABB {
    //     self.hitable.get_bounding_box(t0, t1)
    // }
}
