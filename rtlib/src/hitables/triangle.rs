use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::render::Ray;
use crate::{to_unit_vector, vec3, InnerSpace, Point2, Vector3};
use std::sync::Arc;
use std::{f32, fmt};

pub struct Triangle {
    vertices: Vec<Vector3<f32>>,
    surface_normal: Vector3<f32>,
    material_id: u64,
}

impl Triangle {
    pub fn new(vertices: Vec<Vector3<f32>>, material_id: u64) -> ThreadHitable {
        if vertices.len() != 3 {
            panic!("triangle must have exactly 3 vertices");
        }

        let surface_normal =
            to_unit_vector((vertices[2] - vertices[1]).cross(vertices[1] - vertices[0]));

        Arc::new(Box::new(Triangle {
            vertices,
            surface_normal,
            material_id,
        }))
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Triangle(vertices: {:?}, surface_normal: {:?})]",
            self.vertices, self.surface_normal
        )
    }
}

impl Hitable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        info!("triangle::hit()");

        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let dir = ray.get_direction();
        let pvec = dir.cross(e2);
        let det = e1.dot(pvec);

        if det > -0.0001 && det < 0.0001 {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = ray.get_origin() - self.vertices[0];
        let u = tvec.dot(pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(e1);
        let v = dir.dot(qvec) * inv_det;

        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = e2.dot(qvec) * inv_det;
        if t > 0.0001 && t < t_max && t > t_min {
            return Some(HitRecord::new(
                t,
                ray.get_point_at_parameter(t),
                self.surface_normal,
                self.material_id,
                Point2::new(u, v),
            ));
        }

        return None;
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        let mut min = vec3(
            self.vertices[0].x.min(self.vertices[1].x),
            self.vertices[0].y.min(self.vertices[1].y),
            self.vertices[0].z.min(self.vertices[1].z),
        );
        min = vec3(
            min.x.min(self.vertices[2].x),
            min.y.min(self.vertices[2].y),
            min.z.min(self.vertices[2].z),
        );

        let mut max = vec3(
            self.vertices[0].x.max(self.vertices[1].x),
            self.vertices[0].y.max(self.vertices[1].y),
            self.vertices[0].z.max(self.vertices[1].z),
        );
        max = vec3(
            max.x.max(self.vertices[2].x),
            max.y.max(self.vertices[2].y),
            max.z.max(self.vertices[2].z),
        );

        AABB::new(min, max)
    }

    fn get_pdf_value(&self, _origin: Vector3<f32>, _v: Vector3<f32>) -> f32 {
        1.0
    }

    fn random(&self, _origin: Vector3<f32>) -> Vector3<f32> {
        Vector3::unit_x()
    }
}
