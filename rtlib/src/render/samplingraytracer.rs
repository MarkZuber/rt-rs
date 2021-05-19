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
        the_scene: &Scene,
        render_config: &RenderConfig,
        depth: u32,
    ) -> Color {
        info!("get_ray_color depth: {}", depth);

        // the 0.001 corrects for the "shadow acne"
        match the_scene.get_world().hit(ray, 0.001, std::f32::MAX) {
            Some(hit_record) => {
                let material = the_scene
                    .get_material(&hit_record.get_material_id())
                    .unwrap();
                let emitted = material.emitted(ray, &hit_record);

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
                                let plight = Arc::new(HitablePdf::new(
                                    the_scene.get_light_hitable(),
                                    hit_record.get_p(),
                                    Vector3::new(0.0, 0.0, 0.0),
                                ));
                                let p = MixturePdf::new(plight, scatter_result.get_pdf());
                                let scattered = Ray::new(hit_record.get_p(), p.generate());

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

                emitted
            }
            None => the_scene.get_background_color(),
        }
    }
}
