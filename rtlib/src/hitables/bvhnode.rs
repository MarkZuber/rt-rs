use crate::hitables::{HitRecord, Hitable, ThreadHitable, AABB};
use crate::render::Ray;
use crate::{next_rand_f32, vec3, Vector3};
use std::fmt;
use std::sync::Arc;

pub struct BvhNode {
    left: ThreadHitable,
    right: ThreadHitable,
    bounding_box: Arc<Box<AABB>>,
}

fn build_left_right(
    hitables: &mut [ThreadHitable],
    time_0: f32,
    time_1: f32,
) -> (ThreadHitable, ThreadHitable) {
    match hitables.len() {
        1 => (hitables[0].clone(), hitables[0].clone()),
        2 => (hitables[0].clone(), hitables[1].clone()),
        _ => {
            let len = hitables.len();
            (
                BvhNode::new(&mut hitables[..(len / 2)], time_0, time_1),
                BvhNode::new(&mut hitables[(len / 2)..], time_0, time_1),
            )
        }
    }
}

fn compare_hitables(
    x: ThreadHitable,
    y: ThreadHitable,
    f: impl Fn(Arc<Box<AABB>>, Arc<Box<AABB>>) -> std::cmp::Ordering,
) -> std::cmp::Ordering {
    f(x.get_bounding_box(0.0, 0.0), y.get_bounding_box(0.0, 0.0))
}

impl fmt::Display for BvhNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[BvhNode(aabb: <{}> ==> <LEFT:{}, RIGHT:{}>)]  ",
            self.bounding_box, self.left, self.right
        )
    }
}

impl BvhNode {
    pub fn new(hitables: &mut [ThreadHitable], time_0: f32, time_1: f32) -> ThreadHitable {
        let axis = (3.0 * next_rand_f32()) as i32;
        match axis {
            0 => hitables.sort_by(|a, b| {
                compare_hitables(
                    a.clone(),
                    b.clone(),
                    |l: Arc<Box<AABB>>, r: Arc<Box<AABB>>| {
                        if l.min.x - r.min.x < 0.0 {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    },
                )
            }),
            1 => hitables.sort_by(|a, b| {
                compare_hitables(
                    a.clone(),
                    b.clone(),
                    |l: Arc<Box<AABB>>, r: Arc<Box<AABB>>| {
                        if l.min.y - r.min.y < 0.0 {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    },
                )
            }),
            _ => hitables.sort_by(|a, b| {
                compare_hitables(
                    a.clone(),
                    b.clone(),
                    |l: Arc<Box<AABB>>, r: Arc<Box<AABB>>| {
                        if l.min.z - r.min.z < 0.0 {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    },
                )
            }),
        }

        let (left, right) = build_left_right(hitables, time_0, time_1);

        let box_left = left.get_bounding_box(time_0, time_1);
        let box_right = right.get_bounding_box(time_0, time_1);
        let bounding_box = box_left.get_surrounding_box(box_right);

        Arc::new(Box::new(BvhNode {
            left,
            right,
            bounding_box,
        }))
    }
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        info!("bvhnode::hit()");
        if !self.bounding_box.hit(ray, t_min, t_max) {
            info!("exiting bvhhode::hit() -- NONE");
            return None;
        }

        info!("going left");
        let h_l = self.left.hit(ray, t_min, t_max);
        info!("done left, going right");
        let h_r = self.right.hit(ray, t_min, t_max);
        info!("done right");

        let h = match (h_l, h_r) {
            (Some(hr_left), Some(hr_right)) => {
                if hr_left.get_t() < hr_right.get_t() {
                    info!("hr_left");
                    Some(hr_left)
                } else {
                    info!("hr_right");
                    Some(hr_right)
                }
            }
            (Some(hr_left), None) => {
                info!("hr_left");
                Some(hr_left)
            }
            (None, Some(hr_right)) => {
                info!("hr_right");
                Some(hr_right)
            }
            (None, None) => {
                info!("none/none");
                None
            }
        };

        info!("exiting bvhhode::hit()");

        h
    }
    fn get_pdf_value(&self, _origin: Vector3<f32>, _v: Vector3<f32>) -> f32 {
        0.0
    }
    fn random(&self, _origin: Vector3<f32>) -> Vector3<f32> {
        vec3(0.0, 0.0, 0.0)
    }

    fn get_bounding_box(&self, _t0: f32, _t1: f32) -> Arc<Box<AABB>> {
        self.bounding_box.clone()
    }
}
