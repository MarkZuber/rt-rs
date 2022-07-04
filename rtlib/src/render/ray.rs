use cgmath::Vector3;

use crate::stats::RenderStats;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, stat: &mut RenderStats) -> Ray {
        stat.ray_create();
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
