use crate::get_random_in_unit_sphere;
use crate::hitables::HitRecord;
use crate::materials::{Material, ScatterResult, ThreadMaterial};
use crate::render::{Color, Ray};
use crate::textures::ThreadTexture;
use std::f32;
use std::sync::Arc;

pub struct IsotropicMaterial {
    albedo: ThreadTexture,
}

impl IsotropicMaterial {
    pub fn new(albedo: ThreadTexture) -> ThreadMaterial {
        Arc::new(Box::new(IsotropicMaterial { albedo }))
    }
}

impl Material for IsotropicMaterial {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Arc<Box<ScatterResult>> {
        let scattered = Ray::new(hit_record.get_p(), get_random_in_unit_sphere());
        let attenuation = self
            .albedo
            .get_value(hit_record.get_uv_coords(), hit_record.get_p());
        Arc::new(Box::new(ScatterResult::new(
            true,
            attenuation,
            Some(scattered),
            None,
        )))
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }

    fn emitted(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Color {
        Color::zero()
    }
}
