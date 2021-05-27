use serde::Serialize;
use std::fmt;
use std::sync::Mutex;

pub enum RenderStat {
    RayCreate,
    CameraRayCreate,
    AabbHit,
    BvhNodeHit,
    CubeHit,
    HitableListHit,
    MediumHit,
    SphereHit,
    TriangleHit,
    XyRectHit,
    XzRectHit,
    YzRectHit,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct RenderStats {
    ray_creates: u64,
    camera_ray_creates: u64,
    aabb_hits: u64,
    bvh_node_hits: u64,
    cube_hits: u64,
    hitable_list_hits: u64,
    medium_hits: u64,
    sphere_hits: u64,
    triangle_hits: u64,
    xy_rect_hits: u64,
    xz_rect_hits: u64,
    yz_rect_hits: u64,
}

impl fmt::Display for RenderStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let j = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", j)
    }
}

lazy_static! {
    static ref GLOBAL_STATS: Mutex<RenderStats> = Mutex::new(RenderStats::default());
}

pub fn reset_stats() {
    let mut guard = GLOBAL_STATS.lock().unwrap();
    *guard = RenderStats::default();
}

pub fn get_stats() -> RenderStats {
    GLOBAL_STATS.lock().unwrap().clone()
}

pub fn record_stat(stat: RenderStat) {
    let mut glob = GLOBAL_STATS.lock().unwrap();
    match stat {
        RenderStat::RayCreate => {
            glob.ray_creates += 1;
        }
        RenderStat::CameraRayCreate => {
            glob.camera_ray_creates += 1;
        }
        RenderStat::AabbHit => {
            glob.aabb_hits += 1;
        }
        RenderStat::BvhNodeHit => {
            glob.bvh_node_hits += 1;
        }
        RenderStat::CubeHit => {
            glob.cube_hits += 1;
        }
        RenderStat::HitableListHit => {
            glob.hitable_list_hits += 1;
        }
        RenderStat::MediumHit => {
            glob.medium_hits += 1;
        }
        RenderStat::SphereHit => {
            glob.sphere_hits += 1;
        }
        RenderStat::TriangleHit => {
            glob.triangle_hits += 1;
        }
        RenderStat::XyRectHit => {
            glob.xy_rect_hits += 1;
        }
        RenderStat::XzRectHit => {
            glob.xz_rect_hits += 1;
        }
        RenderStat::YzRectHit => {
            glob.yz_rect_hits += 1;
        }
    }
}
