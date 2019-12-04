use crate::pdfs::Pdf;
use crate::Vector3;
use rand::{thread_rng, Rng};
use std::sync::Arc;

pub struct MixturePdf {
    p0: Arc<dyn Pdf>,
    p1: Option<Arc<dyn Pdf>>,
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Option<Arc<dyn Pdf>>) -> MixturePdf {
        MixturePdf { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn get_value(&self, direction: Vector3<f32>) -> f32 {
        match &self.p1 {
            Some(p1) => (0.5 * self.p0.get_value(direction)) + (0.5 * p1.get_value(direction)),
            None => self.p0.get_value(direction),
        }
    }

    fn generate(&self) -> Vector3<f32> {
        match &self.p1 {
            Some(p1) => {
                let mut rng = thread_rng();

                if rng.gen::<f32>() < 0.5 {
                    return self.p0.generate();
                }
                return p1.generate();
            }
            None => {
                return self.p0.generate();
            }
        }
    }
}
