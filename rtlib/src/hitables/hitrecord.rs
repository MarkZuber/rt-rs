use crate::{Point2, Vector3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material_id: u64,

    // Texture Coordinates
    pub uv_coords: Point2<f32>,
}

impl HitRecord {
    pub fn new(
        t: f32,
        p: Vector3<f32>,
        normal: Vector3<f32>,
        material_id: u64,
        uv_coords: Point2<f32>,
    ) -> HitRecord {
        HitRecord {
            t,
            p,
            normal,
            material_id,
            uv_coords,
        }
    }

    pub fn get_uv_coords(&self) -> Point2<f32> {
        self.uv_coords
    }

    pub fn get_p(&self) -> Vector3<f32> {
        self.p
    }

    pub fn get_normal(&self) -> Vector3<f32> {
        self.normal
    }

    pub fn get_material_id(&self) -> u64 {
        self.material_id
    }
}
