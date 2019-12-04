use crate::hitables::HitRecord;
use crate::materials::Material;
use crate::materials::ScatterResult;
use crate::render::Color;
use crate::render::Ray;
use crate::textures::ColorTexture;
use crate::textures::Texture;
use crate::{Point2, Vector3};
use std::sync::Arc;

pub struct DiffuseLight {
    texture: Arc<Box<ColorTexture>>,
}

impl DiffuseLight {
    pub fn new(texture: Arc<Box<ColorTexture>>) -> DiffuseLight {
        DiffuseLight { texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Arc<Box<ScatterResult>> {
        Arc::new(Box::new(ScatterResult::new_false()))
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }

    fn emitted(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        uv_coords: Point2<f32>,
        p: Vector3<f32>,
    ) -> Color {
        // let dot = hit_record.get_normal().dot(ray_in.get_direction());

        let color = self.texture.get_value(uv_coords, p);
        println!("Emitted: {:?}", color);

        color

        // if dot < 0.0 {
        //     return self.texture.get_value(uv_coords, p);
        // }
        // return self.texture.get_value(uv_coords, p);
    }
}
