use crate::hitables::{
    FlipNormals, HitRecord, Hitable, HitableList, ThreadHitable, XyRect, XzRect, YzRect,
};
use crate::render::Ray;
use crate::Vector3;
use std::sync::Arc;

pub struct Cube {
    _pos_min: Vector3<f32>,
    _pos_max: Vector3<f32>,
    list: ThreadHitable,
}

impl Cube {
    pub fn new(p0: Vector3<f32>, p1: Vector3<f32>, material_id: u64) -> ThreadHitable {
        let list = HitableList::from_vec(vec![
            XyRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, material_id),
            FlipNormals::new(XyRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, material_id)),
            XzRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, material_id),
            FlipNormals::new(XzRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, material_id)),
            YzRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, material_id),
            FlipNormals::new(YzRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, material_id)),
        ]);
        Arc::new(Box::new(Cube {
            _pos_min: p0,
            _pos_max: p1,
            list: list,
        }))
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.list.hit(ray, t_min, t_max)
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        self.list.get_pdf_value(origin, v)
    }
    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        self.list.random(origin)
    }

    // fn get_bounding_box(&self, t0: f32, t1: f32) -> AABB {
    // return new AABB(PosMin, PosMax);
    // }
}
