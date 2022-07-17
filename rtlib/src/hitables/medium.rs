use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::materials::{CompiledMaterials, IsotropicMaterial};
use crate::next_rand_f32;
use crate::render::Ray;
use crate::stats::RenderStats;
use crate::textures::ThreadTexture;
use crate::{InnerSpace, Point2, Vector3};
use std::sync::Arc;
use std::{f32, fmt};

pub struct ConstantMedium {
    boundary: ThreadHitable,
    density: f32,
    phase_function: u64,
}

impl ConstantMedium {
    pub fn new(
        boundary: ThreadHitable,
        density: f32,
        texture: ThreadTexture,
        materials: &mut CompiledMaterials,
    ) -> ThreadHitable {
        let phase_function = materials.add(IsotropicMaterial::new(texture));
        Arc::new(Box::new(ConstantMedium {
            boundary,
            density,
            phase_function,
        }))
    }
}

impl fmt::Display for ConstantMedium {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ConstantMedium(boundary: {}, density: {})]",
            self.boundary, self.density
        )
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, stat: &mut RenderStats) -> Option<HitRecord> {
        info!("ConstantMedium::hit()");
        stat.medium_hit();

        if let Some(hit_record1) = self.boundary.hit(ray, -f32::MAX, f32::MAX, stat) {
            if let Some(hit_record2) =
                self.boundary
                    .hit(ray, hit_record1.get_t() + 0.0001, f32::MAX, stat)
            {
                let mut rec1t = hit_record1.get_t();
                let mut rec2t = hit_record2.get_t();
                if rec1t < t_min {
                    rec1t = t_min;
                }
                if rec2t > t_max {
                    rec2t = t_max;
                }
                if rec1t >= rec2t {
                    return None;
                }
                if rec1t < 0.0 {
                    rec1t = 0.0;
                }
                let distance_inside_boundary = ((rec2t - rec1t) * ray.get_direction()).magnitude();
                let hit_distance = -(1.0 / self.density) * next_rand_f32().ln();
                if hit_distance < distance_inside_boundary {
                    let rec_t = rec1t + (hit_distance / ray.get_direction().magnitude());
                    return Some(HitRecord::new(
                        rec_t,
                        ray.get_point_at_parameter(rec_t),
                        Vector3::unit_x(), // arbitrary
                        0.0,               // todo: distance_squared
                        self.phase_function,
                        Point2::new(0.0, 0.0), // don't need u/v since PhaseFunction is a calculation
                    ));
                }
            }
        }

        return None;
    }

    fn get_bounding_box(&self, t0: f32, t1: f32) -> Arc<Box<AABB>> {
        self.boundary.get_bounding_box(t0, t1)
    }

    fn get_pdf_value(
        &self,
        _origin: Vector3<f32>,
        _v: Vector3<f32>,
        _stat: &mut RenderStats,
    ) -> f32 {
        1.0
    }

    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        self.boundary.random(origin)
    }
}
