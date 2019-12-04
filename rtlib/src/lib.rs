pub mod cameras;
pub mod hitables;
pub mod materials;
pub mod pdfs;
pub mod render;
pub mod textures;

pub use cgmath::{vec3, InnerSpace, Point2, Vector3};

use rand::Rng;
use std::f32;

fn to_unit_vector(v: Vector3<f32>) -> Vector3<f32> {
    v / v.magnitude()
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    let z = 1.0_f32 + (r2 * ((1.0_f32 - (radius * radius / distance_squared)).sqrt() - 1.0_f32));
    let phi = 2.0_f32 * f32::consts::PI * r1;
    let x = phi.cos() * (1.0_f32 - (z * z)).sqrt();
    let y = phi.sin() * (1.0_f32 - (z * z)).sqrt();
    Vector3 { x, y, z }
}
