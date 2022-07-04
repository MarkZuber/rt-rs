use crate::next_rand_f32;
use crate::pdfs::Pdf;
use crate::stats::RenderStats;
use crate::Vector3;
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
    fn get_value(&self, direction: Vector3<f32>, stat: &mut RenderStats) -> f32 {
        match &self.p1 {
            Some(p1) => {
                (0.5 * self.p0.get_value(direction, stat)) + (0.5 * p1.get_value(direction, stat))
            }
            None => self.p0.get_value(direction, stat),
        }
    }

    fn generate(&self) -> Vector3<f32> {
        match &self.p1 {
            Some(p1) => {
                if next_rand_f32() < 0.5 {
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
