use crate::hitables::HitRecord;
use crate::materials::{Material, ScatterResult, ThreadMaterial};
use crate::render::{Color, Ray};
use crate::stats::RenderStats;
use crate::textures::ThreadTexture;
use crate::InnerSpace;
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
    fn scatter(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _stat: &mut RenderStats,
    ) -> Arc<Box<ScatterResult>> {
        Arc::new(Box::new(ScatterResult::new_false()))
    }

    fn emitted(&self, ray_in: &Ray, hit_record: &HitRecord) -> Color {
        if hit_record.get_normal().dot(ray_in.get_direction()) < 0.0 {
            return self
                .texture
                .get_value(hit_record.get_uv_coords(), hit_record.get_p());
        } else {
            return Color::zero();
        }
    }
}
