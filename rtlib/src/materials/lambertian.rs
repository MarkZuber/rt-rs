use crate::hitables::HitRecord;
use crate::materials::{Material, ScatterResult, ThreadMaterial};
use crate::pdfs::CosinePdf;
use crate::render::{Color, Ray};
use crate::textures::ThreadTexture;
use crate::{InnerSpace, Point2, Vector3};
use std::f32;
use std::sync::Arc;

pub struct LambertianMaterial {
    albedo: ThreadTexture,
}

impl LambertianMaterial {
    pub fn new(albedo: ThreadTexture) -> ThreadMaterial {
        Arc::new(Box::new(LambertianMaterial { albedo }))
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Arc<Box<ScatterResult>> {
        let attenuation = self
            .albedo
            .get_value(hit_record.get_uv_coords(), hit_record.get_p());
        let pdf = Arc::new(CosinePdf::new(hit_record.get_normal()));
        Arc::new(Box::new(ScatterResult::new(
            true,
            attenuation,
            None,
            Some(pdf),
        )))
    }

    fn scattering_pdf(&self, _ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = hit_record
            .get_normal()
            .dot(scattered.get_direction().normalize());
        if cosine < 0.0_f32 {
            return 0.0_f32;
        }

        return cosine / f32::consts::PI;
    }

    fn emitted(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _uv_coords: Point2<f32>,
        _p: Vector3<f32>,
    ) -> Color {
        Color::zero()
    }
}
