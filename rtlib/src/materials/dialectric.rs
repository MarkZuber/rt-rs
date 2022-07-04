use crate::hitables::HitRecord;
use crate::materials::{reflect, refract, Material, ScatterResult, ThreadMaterial};
use crate::next_rand_f32;
use crate::render::{Color, Ray};
use crate::stats::RenderStats;
use crate::{vec3, InnerSpace};
use std::f32;
use std::sync::Arc;

pub struct DialectricMaterial {
    refraction_index: f32,
}

impl DialectricMaterial {
    pub fn new(refraction_index: f32) -> ThreadMaterial {
        Arc::new(Box::new(DialectricMaterial { refraction_index }))
    }

    fn calculate_schlick_approximation(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * (1.0 - cosine).powf(5.0))
    }
}

impl Material for DialectricMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        stat: &mut RenderStats,
    ) -> Arc<Box<ScatterResult>> {
        let reflected = reflect(ray_in.get_direction(), hit_record.get_normal());
        let attenuation = Color::one();
        let (outward_normal, ni_over_nt, cosine) =
            if ray_in.get_direction().dot(hit_record.get_normal()) > 0.0 {
                (
                    -hit_record.get_normal(),
                    self.refraction_index,
                    self.refraction_index * ray_in.get_direction().dot(hit_record.get_normal())
                        / ray_in.get_direction().magnitude(),
                )
            } else {
                (
                    hit_record.get_normal(),
                    (1.0 / self.refraction_index),
                    -ray_in.get_direction().dot(hit_record.get_normal())
                        / ray_in.get_direction().magnitude(),
                )
            };

        let refracted = refract(ray_in.get_direction(), outward_normal, ni_over_nt);

        let scattered = if refracted != vec3(0.0, 0.0, 0.0) {
            let reflect_probability =
                DialectricMaterial::calculate_schlick_approximation(cosine, self.refraction_index);
            if next_rand_f32() < reflect_probability {
                Ray::new(hit_record.get_p(), reflected, stat)
            } else {
                Ray::new(hit_record.get_p(), refracted, stat)
            }
        } else {
            Ray::new(hit_record.get_p(), reflected, stat)
        };

        Arc::new(Box::new(ScatterResult::new(
            true,
            attenuation,
            Some(scattered),
            None,
        )))
    }
}
