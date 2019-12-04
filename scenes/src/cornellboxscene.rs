use rtlib::cameras::{Camera, NormalCamera};
use rtlib::hitables::{Hitable, HitableList, Sphere};
use rtlib::materials::{CompiledMaterials, DiffuseLight, LambertianMaterial};
use rtlib::render::Scene;
use rtlib::textures::ColorTexture;
use rtlib::{vec3, Vector3};
use std::sync::Arc;

pub fn create_cornell_box_scene() -> Scene {
    let mut materials: Box<CompiledMaterials> = Box::new(CompiledMaterials::new());

    let matid1: u64 = materials.add(Arc::new(Box::new(LambertianMaterial::new(Arc::new(
        Box::new(ColorTexture::new(0.65, 0.00, 0.05)),
    )))));
    let matid2: u64 = materials.add(Arc::new(Box::new(LambertianMaterial::new(Arc::new(
        Box::new(ColorTexture::new(0.00, 0.00, 0.65)),
    )))));
    let sphere = Sphere::new(vec3(190.0, 90.0, 190.0), 90.0, matid1);
    let sphere2 = Sphere::new(vec3(75.0, 90.0, 190.0), 75.0, matid2);

    let hitables = HitableList::from_vec(vec![Box::new(sphere), Box::new(sphere2)]);

    let world: Arc<Box<dyn Hitable + Send>> = Arc::new(Box::new(hitables));
    let lightid: u64 = materials.add(Arc::new(Box::new(DiffuseLight::new(Arc::new(Box::new(
        ColorTexture::new(15.0, 15.0, 15.0),
    ))))));
    let light_hitable: Arc<Box<dyn Hitable + Send>> = Arc::new(Box::new(Sphere::new(
        vec3(213.0, 343.0, 227.0),
        150.0,
        lightid,
    )));
    let scene = Scene::new(world, light_hitable, Arc::new(materials));

    scene
}

pub fn create_cornell_box_camera(image_width: u32, image_height: u32) -> Box<dyn Camera> {
    // todo: obviously need to move this...
    let look_from = vec3(278.0, 278.0, -800.0);
    let look_at = vec3(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    Box::new(NormalCamera::new(
        look_from,
        look_at,
        Vector3::unit_y(),
        40.0,
        image_width as f32 / image_height as f32,
        aperture,
        dist_to_focus,
    ))
}
