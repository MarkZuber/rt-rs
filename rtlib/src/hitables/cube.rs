use crate::hitables::{
    FlipNormals, HitRecord, Hitable, HitableList, ThreadHitable, XyRect, XzRect, YzRect, AABB,
};
use crate::render::Ray;
use crate::stats::RenderStats;
use crate::Vector3;
use std::fmt;
use std::sync::Arc;

pub struct Cube {
    pos_min: Vector3<f32>,
    pos_max: Vector3<f32>,
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
            pos_min: p0,
            pos_max: p1,
            list: list,
        }))
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Cube({:?}, {:?})]", self.pos_min, self.pos_max)
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, stat: &mut RenderStats) -> Option<HitRecord> {
        info!("Cube::hit()");
        stat.cube_hit();
        self.list.hit(ray, t_min, t_max, stat)
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>, stat: &mut RenderStats) -> f32 {
        self.list.get_pdf_value(origin, v, stat)
    }
    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        self.list.random(origin)
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        AABB::new(self.pos_min, self.pos_max)
    }
}
