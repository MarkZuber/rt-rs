use crate::hitables::Hitable;
use crate::materials::CompiledMaterials;
use std::sync::Arc;

pub struct Scene {
    world: Arc<Box<dyn Hitable + Send>>,
    light_hitable: Arc<Box<dyn Hitable + Send>>,
    materials: Arc<Box<CompiledMaterials>>,
}

impl Scene {
    pub fn new(
        world: Arc<Box<dyn Hitable + Send>>,
        light_hitable: Arc<Box<dyn Hitable + Send>>,
        materials: Arc<Box<CompiledMaterials>>,
    ) -> Scene {
        Scene {
            world,
            light_hitable,
            materials,
        }
    }

    pub fn get_world(&self) -> Arc<Box<dyn Hitable + Send>> {
        self.world.clone()
    }

    pub fn get_light_hitable(&self) -> Arc<Box<dyn Hitable + Send>> {
        self.light_hitable.clone()
    }

    pub fn get_materials(&self) -> Arc<Box<CompiledMaterials>> {
        self.materials.clone()
    }
}
