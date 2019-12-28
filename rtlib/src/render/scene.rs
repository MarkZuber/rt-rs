use crate::hitables::Hitable;
use crate::materials::{CompiledMaterials, Material};
use crate::render::Color;
use std::sync::Arc;

pub struct Scene {
    world: Arc<Box<dyn Hitable + Send>>,
    light_hitable: Arc<Box<dyn Hitable + Send>>,
    materials: Arc<Box<CompiledMaterials>>,
    background_color: Color,
}

impl Scene {
    pub fn new(
        world: Arc<Box<dyn Hitable + Send>>,
        light_hitable: Arc<Box<dyn Hitable + Send>>,
        materials: Arc<Box<CompiledMaterials>>,
        background_color: Color,
    ) -> Scene {
        Scene {
            world,
            light_hitable,
            materials,
            background_color,
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

    pub fn get_material(&self, id: &u64) -> Option<Arc<Box<dyn Material + Send>>> {
        self.materials.get_material(id)
    }

    pub fn get_background_color(&self) -> Color {
        self.background_color
    }
}
