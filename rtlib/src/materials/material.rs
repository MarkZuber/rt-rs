use crate::hitables::HitRecord;
use crate::materials::ScatterResult;
use crate::render::{Color, Ray};
use crate::{Point2, Vector3};
use std::collections::HashMap;

use std::sync::Arc;

pub trait Material: Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Arc<Box<ScatterResult>>;
    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32;
    fn emitted(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        uv_coords: Point2<f32>,
        p: Vector3<f32>,
    ) -> Color;
}

pub struct CompiledMaterial {
    pub material: Arc<Box<dyn Material + Send>>,
    pub id: u64,
}

impl CompiledMaterial {
    pub fn new(material: Arc<Box<dyn Material + Send>>, id: u64) -> CompiledMaterial {
        CompiledMaterial { material, id }
    }
}

pub struct CompiledMaterials {
    next_id: u64,
    materials: HashMap<u64, Arc<CompiledMaterial>>,
}

impl CompiledMaterials {
    pub fn new() -> CompiledMaterials {
        CompiledMaterials {
            next_id: 0,
            materials: HashMap::new(),
        }
    }

    pub fn add(&mut self, material: Arc<Box<dyn Material + Send>>) -> u64 {
        let id = self.next_id;
        self.materials
            .insert(id, Arc::new(CompiledMaterial::new(material, id)));
        self.next_id = self.next_id + 1;
        return id;
    }

    pub fn get_material(&self, id: &u64) -> Option<Arc<Box<dyn Material + Send>>> {
        let compiled_mat: Option<&Arc<CompiledMaterial>> = self.materials.get(id);
        match compiled_mat {
            Some(compiled_mat) => {
                return Some(compiled_mat.material.clone());
            }
            None => {
                return None;
            }
        }
    }
}
