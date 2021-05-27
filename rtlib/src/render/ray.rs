use crate::stats::{record_stat, RenderStat};
use cgmath::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        record_stat(RenderStat::RayCreate);
        Ray { origin, direction }
    }

    pub fn get_point_at_parameter(self, t: f32) -> Vector3<f32> {
        self.origin + (t * self.direction)
    }

    pub fn get_direction(self) -> Vector3<f32> {
        self.direction
    }

    pub fn get_origin(self) -> Vector3<f32> {
        self.origin
    }
}
