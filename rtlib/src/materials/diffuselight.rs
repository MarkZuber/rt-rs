use crate::hitables::HitRecord;
use crate::materials::{Material, ScatterResult, ThreadMaterial};
use crate::render::{Color, Ray};
use crate::textures::ThreadTexture;
use crate::{Point2, Vector3};
use std::sync::Arc;

pub struct DiffuseLight {
    texture: ThreadTexture,
}

impl DiffuseLight {
    pub fn new(texture: ThreadTexture) -> ThreadMaterial {
        Arc::new(Box::new(DiffuseLight { texture }))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Arc<Box<ScatterResult>> {
        Arc::new(Box::new(ScatterResult::new_false()))
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }

    fn emitted(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        uv_coords: Point2<f32>,
        p: Vector3<f32>,
    ) -> Color {
        self.texture.get_value(uv_coords, p)
    }
}
