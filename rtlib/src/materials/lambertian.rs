// use crate::get_random_in_unit_sphere;
use crate::hitables::HitRecord;
use crate::materials::{Material, ScatterResult, ThreadMaterial};
use crate::pdfs::CosinePdf;
use crate::render::{Color, Ray};
use crate::textures::ThreadTexture;
use crate::{get_random_in_unit_sphere, to_unit_vector, InnerSpace};
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
        let target = hit_record.get_p() + hit_record.get_normal() + get_random_in_unit_sphere();
        let scattered = Ray::new(
            hit_record.get_p(),
            to_unit_vector(target - hit_record.get_p()),
        );

        let attenuation = self
            .albedo
            .get_value(hit_record.get_uv_coords(), hit_record.get_p());

        Arc::new(Box::new(ScatterResult::new(
            true,
            attenuation,
            Some(scattered),
            None,
        )))

        // Arc::new(Box::new(ScatterResult::new(
        //     true,
        //     attenuation,
        //     None,
        //     Some(Arc::new(CosinePdf::new(hit_record.get_normal()))),
        // )))
    }

    fn scattering_pdf(&self, _ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = hit_record
            .get_normal()
            .dot(to_unit_vector(scattered.get_direction()));
        if cosine < 0.0 {
            return 0.0;
        }

        return cosine / f32::consts::PI;
    }

    fn emitted(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Color {
        Color::zero()
    }
}
