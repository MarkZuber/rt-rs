use crate::cameras::ThreadCamera;
use crate::hitables::{BvhNode, HitableList, ThreadHitable};
use crate::materials::CompiledMaterials;
use crate::render::{Color, RenderConfig, Scene};
use std::sync::Arc;

pub trait SceneGenerator: Sync {
    fn get_scene(&self) -> Scene;
    fn get_camera(&self) -> ThreadCamera;
    fn get_camera_angled(&self, angle_x: f32, angle_y: f32) -> ThreadCamera;
    fn get_render_config(&self) -> RenderConfig;
    fn get_background_color(&self) -> Color;
}

pub fn create_scene(
    hitables: &Vec<ThreadHitable>,
    materials: Arc<Box<CompiledMaterials>>,
    light_hitable: &ThreadHitable,
    background_color: Color,
    use_bvh: bool,
) -> Scene {
    let mut hitables = hitables.clone();
    let world = if use_bvh {
        let bvh_world = BvhNode::new(&mut hitables, 0.0, 0.0);
        info!("BVH WORLD: {}", bvh_world);
        bvh_world
    } else {
        HitableList::from_vec(hitables)
    };

    Scene::new(
        world,
        light_hitable.clone(),
        materials.clone(),
        background_color,
    )
}
