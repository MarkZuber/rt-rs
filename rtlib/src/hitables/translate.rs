use crate::hitables::{HitRecord, Hitable, ThreadHitable};
use crate::render::Ray;
use crate::Vector3;
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

impl Hitable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.get_origin() - self.displacement, ray.get_direction());
        if let Some(hr) = self.hitable.hit(&moved_ray, t_min, t_max) {
            return Some(HitRecord::new(
                hr.get_t(),
                hr.get_p() + self.displacement,
                hr.get_normal(),
                hr.get_material_id(),
                hr.get_uv_coords(),
            ));
        }

        None

        // fn get_bounding_box(&self, t0: f32, t1: f32) -> AABB {
        // var box = Hitable.GetBoundingBox(t0, t1);
        // if (box == null)
        // {
        //     return null;
        // }

        // box = new AABB(box.Min + Displacement, box.Max + Displacement);
        // return box;
        // }
    }
}
