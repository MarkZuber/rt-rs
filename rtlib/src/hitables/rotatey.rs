use crate::hitables::{HitRecord, Hitable, ThreadHitable};
use crate::render::Ray;
use crate::{vec3, Vector3};
use std::f32;
use std::sync::Arc;

pub struct RotateY {
    hitable: ThreadHitable,
    sin_theta: f32,
    cos_theta: f32,
    // bounding_box: AABB
}

impl RotateY {
    pub fn new(hitable: ThreadHitable, angle: f32) -> ThreadHitable {
        let radians = (f32::consts::PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        // var box = Hitable.GetBoundingBox(0.0f, 1.0f);
        // var min = new Vector3(float.MaxValue, float.MaxValue, float.MaxValue).ToSingleArray();
        // var max = new Vector3(-float.MaxValue, -float.MaxValue, -float.MaxValue).ToSingleArray();

        // for (int i = 0; i < 2; i++)
        // {
        //     float dubi = Convert.ToSingle(i);
        //     for (int j = 0; j < 2; j++)
        //     {
        //         float dubj = Convert.ToSingle(j);
        //         for (int k = 0; k < 2; k++)
        //         {
        //             float dubk = Convert.ToSingle(k);
        //             float x = (dubi * box.Max.X) + ((1.0f - dubi) * box.Min.X);
        //             float y = (dubj * box.Max.Y) + ((1.0f - dubj) * box.Min.Y);
        //             float z = (dubk * box.Max.Z) + ((1.0f - dubk) * box.Min.Z);
        //             float newx = (CosTheta * x) + (SinTheta * z);
        //             float newz = (-SinTheta * x) + (CosTheta * z);
        //             var tester = new Vector3(newx, y, newz).ToSingleArray();
        //             for (int c = 0; c < 3; c++)
        //             {
        //                 if (tester[c] > max[c])
        //                 {
        //                     max[c] = tester[c];
        //                 }

        //                 if (tester[c] < min[c])
        //                 {
        //                     min[c] = tester[c];
        //                 }
        //             }
        //         }
        //     }
        // }

        // BoundingBox = new AABB(new Vector3(min[0], min[1], min[2]), new Vector3(max[0], max[1], max[2]));

        Arc::new(Box::new(RotateY {
            hitable,
            sin_theta,
            cos_theta,
        }))
    }
}

fn to_single_array(v: Vector3<f32>) -> Vec<f32> {
    vec![v.x, v.y, v.z]
}

impl Hitable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = to_single_array(ray.get_origin());
        let mut dir = to_single_array(ray.get_direction());
        origin[0] = (self.cos_theta * ray.get_origin().x) - (self.sin_theta * ray.get_origin().z);
        origin[2] = (self.sin_theta * ray.get_origin().x) + (self.cos_theta * ray.get_origin().z);
        dir[0] =
            (self.cos_theta * ray.get_direction().x) - (self.sin_theta * ray.get_direction().z);
        dir[2] =
            (self.sin_theta * ray.get_direction().x) + (self.cos_theta * ray.get_direction().z);
        let rotated_ray = Ray::new(
            vec3(origin[0], origin[1], origin[2]),
            vec3(dir[0], dir[1], dir[2]),
        );
        if let Some(hit_record) = self.hitable.hit(&rotated_ray, t_min, t_max) {
            let mut p = to_single_array(hit_record.get_p());
            let mut normal = to_single_array(hit_record.get_normal());
            p[0] =
                (self.cos_theta * hit_record.get_p().x) + (self.sin_theta * hit_record.get_p().z);
            p[2] =
                (-self.sin_theta * hit_record.get_p().x) + (self.cos_theta * hit_record.get_p().z);
            normal[0] = (self.cos_theta * hit_record.get_normal().x)
                + (self.sin_theta * hit_record.get_normal().z);
            normal[2] = (-self.sin_theta * hit_record.get_normal().x)
                + (self.cos_theta * hit_record.get_normal().z);

            return Some(HitRecord::new(
                hit_record.get_t(),
                vec3(p[0], p[1], p[2]),
                vec3(normal[0], normal[1], normal[2]),
                hit_record.get_material_id(),
                hit_record.get_uv_coords(),
            ));
        }

        None
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        self.hitable.get_pdf_value(origin, v)
    }
    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        self.hitable.random(origin)
    }
}
