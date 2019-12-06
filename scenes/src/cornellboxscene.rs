use rtlib::cameras::{Camera, NormalCamera};
use rtlib::hitables::{
    BvhNode, Cube, FlipNormals, HitableList, RotateY, Sphere, Translate, XyRect, XzRect, YzRect,
};
use rtlib::materials::{
    CompiledMaterials, DialectricMaterial, DiffuseLight, LambertianMaterial, MetalMaterial,
};
use rtlib::render::Color;
use rtlib::render::Scene;
use rtlib::textures::ColorTexture;
use rtlib::{vec3, Vector3};
use std::sync::Arc;

pub fn create_cornell_box_scene() -> Scene {
    let mut materials: CompiledMaterials = CompiledMaterials::new();

    let light_material = materials.add(DiffuseLight::new(ColorTexture::new(15.0, 15.0, 15.0)));

    let glass = materials.add(DialectricMaterial::new(1.5));
    let red = materials.add(LambertianMaterial::new(ColorTexture::new(0.65, 0.05, 0.05)));
    let white = materials.add(LambertianMaterial::new(ColorTexture::new(0.73, 0.73, 0.73)));
    let green = materials.add(LambertianMaterial::new(ColorTexture::new(0.12, 0.45, 0.15)));
    let aluminum = materials.add(MetalMaterial::new(Color::new(0.8, 0.85, 0.88), 0.0));
    let light_rect = XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light_material);

    let glass_sphere = Sphere::new(vec3(190.0, 90.0, 190.0), 90.0, glass);

    let mut hitables = vec![
        // let hitables = vec![
        FlipNormals::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, green)),
        YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, red),
        FlipNormals::new(light_rect.clone()),
        FlipNormals::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)), // top
        XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white),
        FlipNormals::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)),
        Translate::new(
            RotateY::new(
                Cube::new(vec3(0.0, 0.0, 0.0), vec3(165.0, 330.0, 165.0), aluminum),
                15.0,
            ),
            vec3(265.0, 0.0, 295.0),
        ),
        glass_sphere,
    ];

    // let world = HitableList::from_vec(hitables);

    // todo: bvhnode is broken, needs debugging.  takes 2x time as not using bounding boxes.
    let bvh_world = BvhNode::new(&mut hitables, 0.0, 0.0);
    let scene = Scene::new(bvh_world, light_rect, Arc::new(Box::new(materials)));
    // let scene = Scene::new(world, light_rect, Arc::new(Box::new(materials)));

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
