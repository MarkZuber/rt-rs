use crate::cameras::ThreadCamera;
use crate::hitables::{BvhNode, HitableList, ThreadHitable};
use crate::materials::CompiledMaterials;
use crate::render::Scene;
use std::sync::Arc;

pub trait SceneGenerator {
    fn create_scene(&self) -> Scene;
    fn create_camera(&self, image_width: u32, image_height: u32) -> ThreadCamera;
}

pub fn create_scene(
    hitables: Vec<ThreadHitable>,
    materials: CompiledMaterials,
    light_hitable: &ThreadHitable,
    use_bvh: bool,
) -> Scene {
    let world = if use_bvh {
        let mut hitables = hitables;
        let bvh_world = BvhNode::new(&mut hitables, 0.0, 0.0);
        info!("BVH WORLD: {}", bvh_world);
        bvh_world
    } else {
        HitableList::from_vec(hitables)
    };

    Scene::new(world, light_hitable.clone(), Arc::new(Box::new(materials)))
}
