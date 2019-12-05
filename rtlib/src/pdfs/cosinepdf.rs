use crate::next_rand_f32;
use crate::pdfs::OrthoNormalBase;
use crate::pdfs::Pdf;
use crate::{vec3, InnerSpace, Vector3};
use std::f32;

pub struct CosinePdf {
    uvw: OrthoNormalBase,
}

impl CosinePdf {
    pub fn new(w: Vector3<f32>) -> CosinePdf {
        let uvw = OrthoNormalBase::from_w(w);
        CosinePdf { uvw }
    }
}

// todo: make this common and don't copy/paste it around
fn to_unit_vector(v: Vector3<f32>) -> Vector3<f32> {
    v / v.magnitude()
}

fn get_random_cosine_direction() -> Vector3<f32> {
    let r1 = next_rand_f32();
    let r2 = next_rand_f32();
    let sqrtr2 = r2.sqrt();
    let z: f32 = (1.0 - r2).sqrt();
    let phi: f32 = 2.0 * f32::consts::PI * r1;
    let x: f32 = phi.cos() * 2.0 * sqrtr2;
    let y: f32 = phi.sin() * 2.0 * sqrtr2;

    vec3(x, y, z)
}

impl Pdf for CosinePdf {
    fn get_value(&self, direction: Vector3<f32>) -> f32 {
        let cosine: f32 = to_unit_vector(direction).dot(self.uvw.w());
        let pi = std::f32::consts::PI;

        // todo: book has this as 1.0f, but that causes NaN due to div by zero
        if cosine > 0.0 {
            return cosine / pi;
        }
        return 1.0;
    }

    fn generate(&self) -> Vector3<f32> {
        self.uvw.local(get_random_cosine_direction())
    }
}
