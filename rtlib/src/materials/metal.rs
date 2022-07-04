use crate::get_random_in_unit_sphere;
use crate::hitables::HitRecord;
use crate::materials::reflect;
use crate::materials::{Material, ScatterResult, ThreadMaterial};
use crate::render::{Color, Ray};
use crate::stats::RenderStats;
use crate::InnerSpace;
use std::f32;
use std::sync::Arc;

pub struct MetalMaterial {
    albedo: Color,
    fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f32) -> ThreadMaterial {
        Arc::new(Box::new(MetalMaterial { albedo, fuzz }))
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        stat: &mut RenderStats,
    ) -> Arc<Box<ScatterResult>> {
        let reflected = reflect(ray_in.get_direction().normalize(), hit_record.get_normal());
        let specular_ray = Ray::new(
            hit_record.get_p(),
            reflected + (self.fuzz * get_random_in_unit_sphere()),
            stat,
        );
        let attenuation = self.albedo;

        Arc::new(Box::new(ScatterResult::new(
            true,
            attenuation,
            Some(specular_ray),
            None,
        )))
    }
}
