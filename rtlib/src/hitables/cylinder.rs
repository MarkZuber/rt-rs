use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::pdfs::OrthoNormalBase;
use crate::random_to_sphere;
use crate::render::Ray;
use crate::stats::RenderStats;
use crate::{vec3, InnerSpace, Point2, Vector3};
use std::sync::Arc;
use std::{f32, fmt};

pub struct Cylinder {
    center: Vector3<f32>,
    radius: f32,
    radius_sq: f32,
    half_height: f32,
    material_id: u64,
    material_id_caps: u64,
    bounding_box: Arc<Box<AABB>>,
}

impl Cylinder {
    pub fn new(
        center: Vector3<f32>,
        radius: f32,
        half_height: f32,
        material_id: u64,
        material_id_caps: u64,
    ) -> ThreadHitable {
        // let a = vec3(center.x, center.y, center.z - half_height);
        // let b = vec3(center.x, center.y, center.z + half_height);

        // let kx = (((a.y - b.y).powf(2.0) + (a.z - b.z).powf(2.0))
        //     / ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0) + (a.z - b.z).powf(2.0)))
        // .sqrt();

        // let ky = (((a.x - b.x).powf(2.0) + (a.z - b.z).powf(2.0))
        //     / ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0) + (a.z - b.z).powf(2.0)))
        // .sqrt();
        // let kz = (((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0))
        //     / ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0) + (a.z - b.z).powf(2.0)))
        // .sqrt();

        // let mut min_bb = vec3(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        // let mut max_bb = vec3(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        // min_bb = min_bb - vec3(kx * radius, ky * radius, kz * radius);
        // max_bb = max_bb + vec3(kx * radius, ky * radius, kz * radius);

        // min_bb = min_bb - vec3(0.0, radius, 0.0);
        // max_bb = max_bb + vec3(radius, radius, 0.0);

        // let bounding_box = AABB::new(min_bb, max_bb);

        // todo: fix bounding box
        let bounding_box = AABB::new(
            vec3(-10000.0, -10000.0, -10000.0),
            vec3(10000.0, 10000.0, 10000.0),
        );

        Arc::new(Box::new(Cylinder {
            center: center,
            radius: radius,
            radius_sq: radius * radius,
            half_height,
            material_id,
            material_id_caps,
            bounding_box,
        }))
    }

    pub fn center(&self) -> Vector3<f32> {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn get_sphere_uv(&self, _p: Vector3<f32>) -> Point2<f32> {
        // let pi = std::f32::consts::PI;
        // let punit = to_unit_vector(p);
        // let phi = punit.z.atan2(punit.x);
        // let theta = punit.y.asin();
        // let u = 1.0 - ((phi + pi) / (2.0 * pi));
        // let v = (theta + (pi / 2.0)) / pi;
        // Point2::new(u, v)
        Point2::new(0.0, 0.0)
    }

    pub fn get_material_id(&self) -> u64 {
        self.material_id
    }

    fn check_disk_intersection(
        &self,
        vantage: Vector3<f32>,
        direction: Vector3<f32>,
        z_disc: f32,
    ) -> Option<HitRecord> {
        let u = (z_disc - vantage.z) / direction.z;
        if u > 0.001 {
            // TODO: define epsilon
            let displacement = direction * u;
            let intersect_point = vantage + displacement;
            let x = intersect_point.x;
            let y = intersect_point.y;
            if x * x + y * y <= self.radius_sq {
                let mut z: f32 = 1.0;
                if z_disc < 0.0 {
                    z = -1.0
                }
                let surface_normal = Vector3::new(0.0, 0.0, z);
                // TODO(bug): t is not u, but we don't use t very much, so hacking this in.
                return Some(HitRecord::new(
                    u,
                    intersect_point,
                    surface_normal,
                    displacement.magnitude2(),
                    self.material_id_caps,
                    Point2 { x: 0.0, y: 0.0 },
                ));
            }
        }

        None
    }
}

impl fmt::Display for Cylinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Cylinder(center: {:?}, radius: {})]",
            self.center, self.radius
        )
    }
}

// Returns n=0..numComplexValues, and fills in outArray with the n values from
// inArray that are real-valued (i.e., whose imaginary parts are within TOLERANCE of 0.)
// outArray must be large enough to receive numComplexValues values.
fn filter_real_numbers(in_vec: Vec<num::complex::Complex32>) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::new();

    let tolerance = 1.0e-8;

    for complex_val in in_vec {
        if complex_val.im < tolerance {
            result.push(complex_val.re);
        }
    }

    result
}

fn is_zero(x: num::complex::Complex32) -> bool {
    let tolerance = 1.0e-8;
    (x.re.abs() < tolerance) && (x.im.abs() < tolerance)
}

fn solve_quadratic_equation(
    a: num::complex::Complex32,
    b: num::complex::Complex32,
    c: num::complex::Complex32,
) -> Vec<num::complex::Complex32> {
    if is_zero(a) {
        if is_zero(b) {
            // The equation devolves to: c = 0, where the variable x has vanished!
            return vec![]; // cannot divide by zero, so there is no solution.
        } else {
            // Simple linear equation: bx + c = 0, so x = -c/b.
            return vec![-c / b];
        }
    } else {
        let radicand: num::complex::Complex32 = (b * b) - (4.0 * a * c);
        if is_zero(radicand) {
            // Both roots have the same value: -b / 2a.
            return vec![-b / (a * 2.0)];
        } else {
            // There are two distinct real roots.
            let r: num::complex::Complex32 = radicand.sqrt();
            let d = a * 2.0;

            // return vec![(-b + r) / d, (-b - r) / d];
            // TODO: returning both of these roots gives us a weird arc.  WHY?!
            return vec![(-b - r) / d];
        }
    }
}

fn solve_quadratic_eq(a: f32, b: f32, c: f32) -> Vec<f32> {
    let complex_roots = solve_quadratic_equation(
        num::complex::Complex32::new(a, 0.0),
        num::complex::Complex32::new(b, 0.0),
        num::complex::Complex32::new(c, 0.0),
    );

    return filter_real_numbers(complex_roots);
}

fn pick_closest_hitrecord(hit_records: Vec<HitRecord>) -> Option<HitRecord> {
    info!(
        "cylinder pick_closest_hitrecord length: {}",
        hit_records.len()
    );

    if hit_records.len() == 0 {
        return None;
    }
    if hit_records.len() == 1 {
        return Some(hit_records[0]);
    }

    let mut closest: HitRecord = hit_records[0];

    for i in 1..hit_records.len() {
        let cur_hr = &hit_records[i];
        let diff = cur_hr.distance_squared - closest.distance_squared;
        if diff.abs() < 0.001 {
            // this is a tie, so we'll keep current closest
        } else if diff < 0.0 {
            closest = *cur_hr;
        }
    }
    Some(closest)
}

impl Hitable for Cylinder {
    fn hit(
        &self,
        ray: &Ray,
        _t_min: f32,
        _t_max: f32,
        stat: &mut RenderStats,
    ) -> Option<HitRecord> {
        info!("cylinder::hit()");
        stat.cylinder_hit();

        let mut intersections: Vec<HitRecord> = Vec::new();

        if ray.get_direction().z.abs() > 0.001 {
            if let Some(x) = self.check_disk_intersection(
                ray.get_origin(),
                ray.get_direction(),
                self.half_height,
            ) {
                info!("cylinder::disc_hit 1");
                intersections.push(x);
            }
            if let Some(x) = self.check_disk_intersection(
                ray.get_origin(),
                ray.get_direction(),
                -self.half_height,
            ) {
                info!("cylinder::disc_hit 2");
                intersections.push(x);
            }
        }

        let roots = solve_quadratic_eq(
            ray.get_direction().x.powf(2.0) + ray.get_direction().y.powf(2.0),
            2.0 * (ray.get_origin().x * ray.get_direction().x
                + ray.get_origin().y * ray.get_direction().y),
            (ray.get_origin().x.powf(2.0) + ray.get_origin().y.powf(2.0)) - self.radius_sq,
        );

        for root in roots {
            if root > 0.001 {
                let displacement = ray.get_direction() * root;
                let intersect_point = ray.get_origin() + displacement;

                if intersect_point.z.abs() <= self.half_height {
                    let distance_squared = displacement.magnitude2();
                    let surface_normal =
                        vec3(intersect_point.x, intersect_point.y, 0.0).normalize();
                    // todo(hack): faking "t"
                    info!("cylinder main intersection");

                    intersections.push(HitRecord::new(
                        0.0,
                        intersect_point,
                        surface_normal,
                        distance_squared,
                        self.material_id,
                        Point2 { x: 0.0, y: 0.0 },
                    ));
                }
            }
        }
        info!("cylinder picking closest hit record");

        pick_closest_hitrecord(intersections)
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        self.bounding_box.clone()
    }

    fn get_pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>, stat: &mut RenderStats) -> f32 {
        match self.hit(&Ray::new(origin, v, stat), 0.001, f32::MAX, stat) {
            Some(_hr) => {
                let centoriginmag2 = (self.center() - origin).magnitude2();
                let cos_theta_max = (1.0 - (self.radius_sq / centoriginmag2)).sqrt();
                let solid_angle = 2.0 * f32::consts::PI * (1.0 - cos_theta_max);
                return 1.0 / solid_angle;
            }
            None => return 0.0,
        }
    }

    fn random(&self, origin: Vector3<f32>) -> Vector3<f32> {
        let direction = self.center() - origin;
        let distance_squared = direction.magnitude2();
        let uvw = OrthoNormalBase::from_w(direction);
        let v = random_to_sphere(self.radius(), distance_squared);
        return uvw.local(v);
    }
}
