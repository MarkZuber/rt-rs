use crate::hitables::HitRecord;
use crate::hitables::Hitable;
use crate::render::Ray;
use crate::{vec3, Point2, Vector3};
use std::sync::Arc;

pub struct HitableList {
    hitables: Arc<Vec<Arc<Box<dyn Hitable + Send>>>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        let hitables: Arc<Vec<Arc<Box<dyn Hitable + Send>>>> = Arc::new(Vec::new());
        HitableList { hitables }
    }

    pub fn from_vec(items: Vec<Box<dyn Hitable + Send>>) -> HitableList {
        let mut pre_hitables: Vec<Arc<Box<dyn Hitable + Send>>> = Vec::new();
        for item in items {
            pre_hitables.push(Arc::new(item));
        }

        let hitables = Arc::new(pre_hitables);
        HitableList { hitables }
    }

    // pub fn add(&mut self, hitable: Arc<Box<dyn Hitable + Send>>) {
    //     self.hitables.push(hitable);
    // }

    pub fn size(&self) -> usize {
        self.hitables.len()
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_something: bool = false;
        let mut final_hitrecord: HitRecord = HitRecord::new(
            t_max,
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            0,
            Point2::new(0.0, 0.0),
        );

        for i in 0..self.size() {
            let hit_record_option = self.hitables[i].hit(ray, t_min, final_hitrecord.t);
            match hit_record_option {
                Some(hit_record) => {
                    hit_something = true;
                    if hit_record.t < final_hitrecord.t {
                        final_hitrecord.p = hit_record.p;
                        final_hitrecord.t = hit_record.t;
                        final_hitrecord.normal = hit_record.normal;
                        final_hitrecord.material_id = hit_record.material_id;
                    }
                }
                None => {}
            }
        }

        if hit_something {
            return Some(final_hitrecord);
        }
        return None;
    }

    // fn get_bounding_box(&self, _t0: f32, _t1: f32) -> AABB {
    //     AABB {}
    // }

    fn get_pdf_value(&self, _origin: Vector3<f32>, _v: Vector3<f32>) -> f32 {
        0.0
    }

    fn random(&self, _origin: Vector3<f32>) -> Vector3<f32> {
        vec3(0.0, 0.0, 0.0)
    }
}
