use crate::hitables::Hitable;
use crate::pdfs::Pdf;
use crate::Vector3;
use std::sync::Arc;

pub struct HitablePdf {
    hitable: Arc<Box<dyn Hitable + Send>>,
    origin: Vector3<f32>,
}

impl HitablePdf {
    pub fn new(
        hitable: Arc<Box<dyn Hitable + Send>>,
        origin: Vector3<f32>,
        _v: Vector3<f32>,
    ) -> HitablePdf {
        HitablePdf { hitable, origin }
    }
}

impl Pdf for HitablePdf {
    fn get_value(&self, direction: Vector3<f32>) -> f32 {
        self.hitable.get_pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vector3<f32> {
        self.hitable.random(self.origin)
    }
}
