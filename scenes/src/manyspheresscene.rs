use rtlib::cameras::{NormalCamera, ThreadCamera};
use rtlib::render::create_scene;

#[allow(unused_imports)]
use rtlib::hitables::{
    BvhNode, Cube, FlipNormals, HitableList, RotateY, Sphere, ThreadHitable, Translate, XyRect,
    XzRect, YzRect,
};
use rtlib::materials::{
    CompiledMaterials, DialectricMaterial, DiffuseLight, LambertianMaterial, MetalMaterial,
};
use rtlib::next_rand_f32;
use rtlib::render::Color;
use rtlib::render::{Scene, SceneGenerator};
use rtlib::textures::{CheckerTexture, ColorTexture};
use rtlib::{vec3, InnerSpace, Vector3};
use std::sync::Arc;

pub struct ManySpheresScene {}

impl ManySpheresScene {
    pub fn new() -> Box<dyn SceneGenerator> {
        Box::new(ManySpheresScene {})
    }
}

impl SceneGenerator for ManySpheresScene {
    fn create_scene(&self) -> Scene {
        let mut materials: CompiledMaterials = CompiledMaterials::new();

        let light_material = materials.add(DiffuseLight::new(ColorTexture::new(15.0, 15.0, 15.0)));

        let checker_texture = CheckerTexture::new(
            ColorTexture::new(0.2, 0.3, 0.1),
            ColorTexture::new(0.9, 0.9, 0.9),
            vec3(10.0, 10.0, 10.0),
        );
        let lambertian_checker_mat = materials.add(LambertianMaterial::new(checker_texture));
        let dialectric_mat = materials.add(DialectricMaterial::new(1.5));

        let lamb_color_mat =
            materials.add(LambertianMaterial::new(ColorTexture::new(0.4, 0.2, 0.1)));
        let metal_color_mat = materials.add(MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0));

        let mut hitables = vec![
            Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, lambertian_checker_mat),
            Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, dialectric_mat),
            Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, lamb_color_mat),
            Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, metal_color_mat),
        ];

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = next_rand_f32();
                let center = vec3(
                    (a as f32) * next_rand_f32(),
                    0.2,
                    (b as f32) + (0.9 * next_rand_f32()),
                );

                if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let rand_mat = materials.add(LambertianMaterial::new(ColorTexture::new(
                            next_rand_f32() * next_rand_f32(),
                            next_rand_f32() * next_rand_f32(),
                            next_rand_f32() * next_rand_f32(),
                        )));
                        hitables.push(Sphere::new(center, 0.2, rand_mat));
                    } else if choose_mat < 0.95 {
                        let metal_mat = materials.add(MetalMaterial::new(
                            Color::new(
                                0.5 * (1.0 + next_rand_f32()),
                                0.5 * (1.0 + next_rand_f32()),
                                0.5 * (1.0 + next_rand_f32()),
                            ),
                            0.5 * next_rand_f32(),
                        ));
                        hitables.push(Sphere::new(center, 0.2, metal_mat));
                    } else {
                        hitables.push(Sphere::new(center, 0.2, dialectric_mat));
                    }
                }
            }
        }

        let light_rect = XzRect::new(-2.0, 2.0, -2.0, 2.0, 5.0, light_material);
        hitables.push(light_rect.clone());

        create_scene(hitables, materials, &light_rect, true)
    }

    fn create_camera(&self, image_width: u32, image_height: u32) -> ThreadCamera {
        let look_from = vec3(24.0, 2.0, 6.0);
        let look_at = Vector3::unit_y();
        let dist_to_focus = (look_from - look_at).magnitude();
        let aperture = 0.1;

        Arc::new(Box::new(NormalCamera::new(
            look_from,
            look_at,
            Vector3::unit_y(),
            15.0,
            image_width as f32 / image_height as f32,
            aperture,
            dist_to_focus,
        )))
    }
}
