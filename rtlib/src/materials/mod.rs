mod dialectric;
mod diffuselight;
mod lambertian;
mod material;
mod metal;
mod scatterresult;

pub use self::dialectric::DialectricMaterial;
pub use self::diffuselight::DiffuseLight;
pub use self::lambertian::LambertianMaterial;
pub use self::material::CompiledMaterial;
pub use self::material::CompiledMaterials;
pub use self::material::Material;
pub use self::material::ThreadMaterial;
pub use self::metal::MetalMaterial;
pub use self::scatterresult::ScatterResult;

use crate::{vec3, InnerSpace, Vector3};

// todo: move these to a trait so we can extend the Vector3 type with them directly.

fn reflect(v: Vector3<f32>, other: Vector3<f32>) -> Vector3<f32> {
    v - (2.0 * v.dot(other) * other)
}

fn refract(v: Vector3<f32>, normal: Vector3<f32>, ni_over_nt: f32) -> Vector3<f32> {
    let unit_vector = crate::to_unit_vector(v);
    let dt = unit_vector.dot(normal);
    let discriminant = 1.0 - (ni_over_nt * ni_over_nt * (1.0 - (dt * dt)));
    if discriminant <= 0.0 {
        return vec3(0.0, 0.0, 0.0);
    }
    return ni_over_nt * (unit_vector - (normal * dt)) - (normal * discriminant.sqrt());
}
