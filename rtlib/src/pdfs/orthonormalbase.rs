use crate::{to_unit_vector, Vector3};

pub struct OrthoNormalBase {
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
}

impl OrthoNormalBase {
    pub fn new(u: Vector3<f32>, v: Vector3<f32>, w: Vector3<f32>) -> OrthoNormalBase {
        OrthoNormalBase { u, v, w }
    }

    pub fn from_w(n: Vector3<f32>) -> OrthoNormalBase {
        let w: Vector3<f32> = to_unit_vector(n);

        let a: Vector3<f32>;
        if w.x.abs() > 0.9 {
            a = Vector3::unit_y();
        } else {
            a = Vector3::unit_x();
        }
        let v: Vector3<f32> = to_unit_vector(w.cross(a));
        let u: Vector3<f32> = w.cross(v);
        OrthoNormalBase::new(u, v, w)
    }

    pub fn u(&self) -> Vector3<f32> {
        self.u
    }

    pub fn v(&self) -> Vector3<f32> {
        self.v
    }

    pub fn w(&self) -> Vector3<f32> {
        self.w
    }

    // pub fn local(&self, a: f32, b: f32, c: f32) -> Vector3<f32> {
    //     return (a * self.w) + (b * self.v) + (c * self.w);
    // }

    pub fn local(&self, a: Vector3<f32>) -> Vector3<f32> {
        return (a.x * self.u) + (a.y * self.v) + (a.z * self.w);
    }
}
