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

fn next_rand_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

fn random_to_sphere(radius: f32, distance_squared: f32) -> Vector3<f32> {
    let r1 = next_rand_f32();
    let r2 = next_rand_f32();
    let z = 1.0_f32 + (r2 * ((1.0_f32 - (radius * radius / distance_squared)).sqrt() - 1.0_f32));
    let phi = 2.0_f32 * f32::consts::PI * r1;
    let x = phi.cos() * (1.0_f32 - (z * z)).sqrt();
    let y = phi.sin() * (1.0_f32 - (z * z)).sqrt();
    Vector3 { x, y, z }
}

fn get_random_in_unit_sphere() -> Vector3<f32> {
    let mut pv: Vector3<f32>;

    loop {
        pv = (2.0 * vec3(next_rand_f32(), next_rand_f32(), next_rand_f32())) - vec3(1.0, 1.0, 1.0);
        if pv.magnitude2() < 1.0 {
            break;
        }
    }

    pv
}
