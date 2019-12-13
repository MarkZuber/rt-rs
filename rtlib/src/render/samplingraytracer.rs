use crate::materials::Material;
use crate::pdfs::{HitablePdf, MixturePdf, Pdf};
use crate::render::{Color, Ray, RayTracer, RenderConfig, Scene};
use crate::Vector3;
use std::sync::Arc;

pub struct SamplingRayTracer {}

impl SamplingRayTracer {
    pub fn new() -> SamplingRayTracer {
        SamplingRayTracer {}
    }
}

impl RayTracer for SamplingRayTracer {
    fn get_ray_color(
        &self,
        ray: &Ray,
        the_scene: Arc<Box<Scene>>,
        render_config: &RenderConfig,
        depth: u32,
    ) -> Color {
        info!("get_ray_color depth: {}", depth);

        // the 0.001 corrects for the "shadow acne"
        match the_scene.get_world().hit(ray, 0.001, std::f32::MAX) {
            Some(hit_record) => {
                let material: Arc<Box<dyn Material + Send>> = the_scene
                    .get_materials()
                    .get_material(&hit_record.get_material_id())
                    .unwrap(); // todo: fix up semantics to remove the unwrap here.
                let emitted = material.emitted(
                    ray,
                    &hit_record,
                    hit_record.get_uv_coords(),
                    hit_record.get_p(),
                );

                // if (hr.Material is DiffuseLight)
                // {
                //     Debug.WriteLine($"HIT A LIGHT. Emitted: {emitted}");
                // }

                if depth < render_config.ray_trace_depth {
                    let scatter_result = material.scatter(ray, &hit_record);
                    if scatter_result.is_scattered() {
                        match scatter_result.get_specular_ray() {
                            Some(specular_ray) => {
                                return scatter_result.get_attenuation().multiply(
                                    self.get_ray_color(
                                        &specular_ray,
                                        the_scene,
                                        render_config,
                                        depth + 1,
                                    ),
                                );
                            }
                            None => {
                                let f = Arc::new(HitablePdf::new(
                                    the_scene.get_light_hitable(),
                                    hit_record.get_p(),
                                    Vector3::new(0.0, 0.0, 0.0),
                                ));
                                let p = MixturePdf::new(f, scatter_result.get_pdf());
                                let scattered = Ray::new(hit_record.get_p(), p.generate());

                                let f = Arc::new(HitablePdf::new(
                                    the_scene.get_light_hitable(),
                                    hit_record.get_p(),
                                    scattered.get_direction(),
                                ));
                                let p = MixturePdf::new(f, scatter_result.get_pdf());
                                let pdf_value = p.get_value(scattered.get_direction());

                                let mut scattering_pdf =
                                    material.scattering_pdf(ray, &hit_record, &scattered);
                                if scattering_pdf < 0.01 {
                                    scattering_pdf = 0.01;
                                }

                                let depth_ray_color = self.get_ray_color(
                                    &scattered,
                                    the_scene,
                                    render_config,
                                    depth + 1,
                                );
                                let recurse_color = (scatter_result
                                    .get_attenuation()
                                    .multiply_by_scalar(scattering_pdf)
                                    .multiply(depth_ray_color))
                                .multiply_by_scalar(1.0 / pdf_value);
                                return emitted.add(recurse_color);
                            }
                        }
                    }
                }

                return emitted;
            }
            None => {
                return Color::new(0.1, 0.1, 0.1); // todo: _backgroundFunc(ray);
            }
        }
    }
}
