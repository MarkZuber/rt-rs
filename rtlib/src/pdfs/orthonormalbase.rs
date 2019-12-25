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
        let w = to_unit_vector(n);

        let a = if w.x.abs() > 0.9 {
            Vector3::unit_y()
        } else {
            Vector3::unit_x()
        };
        let v = to_unit_vector(w.cross(a));
        let u = w.cross(v);
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

    pub fn local(&self, a: Vector3<f32>) -> Vector3<f32> {
        return (a.x * self.u) + (a.y * self.v) + (a.z * self.w);
    }
}
