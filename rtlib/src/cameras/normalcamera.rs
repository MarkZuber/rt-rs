use crate::cameras::Camera;
use crate::get_random_in_unit_sphere;
use crate::render::Ray;
use crate::stats::RenderStats;
use crate::{InnerSpace, Vector3};

pub struct NormalCamera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    lens_radius: f32,
}

impl NormalCamera {
    pub fn new(
        look_from: Vector3<f32>,
        look_at: Vector3<f32>,
        up: Vector3<f32>,
        vertical_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> NormalCamera {
        let lens_radius = aperture / 2.0;
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);
        let lower_left_corner = origin
            - (half_width * focus_distance * u)
            - (half_height * focus_distance * v)
            - (focus_distance * w);
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;

        NormalCamera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
}

impl Camera for NormalCamera {
    fn get_ray(&self, s: f32, t: f32, stat: &mut RenderStats) -> Ray {
        let rd = self.lens_radius * get_random_in_unit_sphere();
        let offset = (self.u * rd.x) + (self.v * rd.y);
        stat.camera_ray_create();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
            stat,
        )
    }
}
