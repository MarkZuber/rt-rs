use rtlib::cameras::{NormalCamera, ThreadCamera};
use rtlib::render::create_scene;

#[allow(unused_imports)]
use rtlib::hitables::{
    BvhNode, Cube, Cylinder, FlipNormals, HitableList, RotateX, RotateY, RotateZ, Sphere,
    ThreadHitable, Translate, XyRect, XzRect, YzRect,
};
use rtlib::materials::{
    CompiledMaterials, DialectricMaterial, DiffuseLight, LambertianMaterial, MetalMaterial,
};
use rtlib::render::Color;
use rtlib::render::{RenderConfig, Scene, SceneGenerator};
use rtlib::textures::{ColorTexture, VectorNoiseMode, VectorNoiseTexture};
use rtlib::{vec3, Vector3};
use std::f32;
use std::sync::Arc;

pub struct CornellBoxScene {
    render_config: RenderConfig,
}

impl CornellBoxScene {
    pub fn new(render_config: &RenderConfig) -> Arc<Box<dyn SceneGenerator + Send>> {
        Arc::new(Box::new(CornellBoxScene {
            render_config: render_config.clone(),
        }))
    }
}

impl SceneGenerator for CornellBoxScene {
    fn get_scene(&self) -> Scene {
        let mut materials: CompiledMaterials = CompiledMaterials::new();

        let light_material = materials.add(DiffuseLight::new(ColorTexture::new(25.0, 25.0, 25.0)));

        let glass = materials.add(DialectricMaterial::new(1.5));
        let red = materials.add(LambertianMaterial::new(ColorTexture::new(0.65, 0.05, 0.05)));
        let white = materials.add(LambertianMaterial::new(ColorTexture::new(0.73, 0.73, 0.73)));
        let noise = materials.add(LambertianMaterial::new(VectorNoiseTexture::new(
            VectorNoiseMode::DarkNoise,
            0.1,
        )));
        let yellow = materials.add(LambertianMaterial::new(ColorTexture::new(1.0, 1.0, 0.0)));
        let blue = materials.add(LambertianMaterial::new(ColorTexture::new(0.05, 0.05, 0.73)));
        let green = materials.add(LambertianMaterial::new(ColorTexture::new(0.12, 0.45, 0.15)));
        let _aluminum = materials.add(MetalMaterial::new(Color::new(0.8, 0.85, 0.88), 0.0));
        let light_rect = FlipNormals::new(XzRect::new(
            213.0,
            343.0,
            227.0,
            332.0,
            554.0,
            light_material,
        ));

        let glass_sphere = Sphere::new(vec3(190.0, 90.0, 190.0), 90.0, glass);

        let cylinder = Cylinder::new(vec3(50.0, 50.0, 50.0), 50.0, 50.0, blue, yellow);

        let hitables = vec![
            FlipNormals::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)),
            YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red),
            light_rect.clone(),
            FlipNormals::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)), // top
            XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white),
            FlipNormals::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, noise)),
            Translate::new(
                RotateY::new(
                    Cube::new(vec3(0.0, 0.0, 0.0), vec3(165.0, 330.0, 165.0), white),
                    15.0,
                ),
                vec3(265.0, 0.0, 295.0),
            ),
            Translate::new(RotateY::new(cylinder, 35.0), vec3(350.0, 100.0, 300.0)),
            glass_sphere,
        ];

        create_scene(
            &hitables,
            Arc::new(Box::new(materials)),
            &light_rect,
            self.get_background_color(),
            true,
        )
    }

    fn get_camera(&self) -> ThreadCamera {
        self.get_camera_angled(0.0, 0.0)
    }

    fn get_camera_angled(&self, angle_x: f32, angle_y: f32) -> ThreadCamera {
        let cam_angle_x = angle_x;
        let cam_angle_y = angle_y;
        let distance = 800.0;

        let cam_x = distance
            * -((cam_angle_x * f32::consts::PI / 180.0).sin())
            * (cam_angle_y * f32::consts::PI / 180.0).cos();
        let cam_y = distance * -((cam_angle_y * f32::consts::PI / 180.0).sin());
        let cam_z = -distance
            * (cam_angle_x * f32::consts::PI / 180.0).cos()
            * (cam_angle_y * f32::consts::PI / 180.0).cos();

        let look_from = vec3(cam_x + 278.0, cam_y + 278.0, cam_z);
        let look_at = vec3(278.0, 278.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.0;

        Arc::new(Box::new(NormalCamera::new(
            look_from,
            look_at,
            Vector3::unit_y(),
            40.0,
            self.render_config.width as f32 / self.render_config.height as f32,
            aperture,
            dist_to_focus,
        )))
    }

    fn get_render_config(&self) -> RenderConfig {
        self.render_config.clone()
    }

    fn get_background_color(&self) -> Color {
        Color::new(0.1, 0.1, 0.1)
    }
}
