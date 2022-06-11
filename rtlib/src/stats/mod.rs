use serde::Serialize;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

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

pub fn reset_stats() {
    STAT_RAY_CREATE.store(0, Ordering::Relaxed);
    STAT_CAMERA_RAY_CREATES.store(0, Ordering::Relaxed);
    STAT_AABB_HITS.store(0, Ordering::Relaxed);
    STAT_BVH_NODE_HITS.store(0, Ordering::Relaxed);
    STAT_CUBE_HITS.store(0, Ordering::Relaxed);
    STAT_HITABLE_LIST_HITS.store(0, Ordering::Relaxed);
    STAT_MEDIUM_HITS.store(0, Ordering::Relaxed);
    STAT_SPHERE_HITS.store(0, Ordering::Relaxed);
    STAT_TRIANGLE_HITS.store(0, Ordering::Relaxed);
    STAT_XY_RECT_HITS.store(0, Ordering::Relaxed);
    STAT_XZ_RECT_HITS.store(0, Ordering::Relaxed);
    STAT_YZ_RECT_HITS.store(0, Ordering::Relaxed);
}

pub fn get_stats() -> RenderStats {
    RenderStats {
        ray_creates: STAT_RAY_CREATE.load(Ordering::Relaxed),
        camera_ray_creates: STAT_CAMERA_RAY_CREATES.load(Ordering::Relaxed),
        aabb_hits: STAT_AABB_HITS.load(Ordering::Relaxed),
        bvh_node_hits: STAT_BVH_NODE_HITS.load(Ordering::Relaxed),
        cube_hits: STAT_CUBE_HITS.load(Ordering::Relaxed),
        hitable_list_hits: STAT_HITABLE_LIST_HITS.load(Ordering::Relaxed),
        medium_hits: STAT_MEDIUM_HITS.load(Ordering::Relaxed),
        sphere_hits: STAT_SPHERE_HITS.load(Ordering::Relaxed),
        triangle_hits: STAT_TRIANGLE_HITS.load(Ordering::Relaxed),
        xy_rect_hits: STAT_XY_RECT_HITS.load(Ordering::Relaxed),
        xz_rect_hits: STAT_XZ_RECT_HITS.load(Ordering::Relaxed),
        yz_rect_hits: STAT_YZ_RECT_HITS.load(Ordering::Relaxed),
    }
}

static STAT_RAY_CREATE: AtomicU64 = AtomicU64::new(0);
static STAT_CAMERA_RAY_CREATES: AtomicU64 = AtomicU64::new(0);
static STAT_AABB_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_BVH_NODE_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_CUBE_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_HITABLE_LIST_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_MEDIUM_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_SPHERE_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_TRIANGLE_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_XY_RECT_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_XZ_RECT_HITS: AtomicU64 = AtomicU64::new(0);
static STAT_YZ_RECT_HITS: AtomicU64 = AtomicU64::new(0);

pub fn record_stat(stat: RenderStat) {
    // match stat {
    //     RenderStat::RayCreate => {
    //         STAT_RAY_CREATE.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::AabbHit => {
    //         STAT_AABB_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::BvhNodeHit => {
    //         STAT_BVH_NODE_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::CameraRayCreate => {
    //         STAT_CAMERA_RAY_CREATES.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::CubeHit => {
    //         STAT_CUBE_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::HitableListHit => {
    //         STAT_HITABLE_LIST_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::MediumHit => {
    //         STAT_MEDIUM_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::SphereHit => {
    //         STAT_SPHERE_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::TriangleHit => {
    //         STAT_TRIANGLE_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::XyRectHit => {
    //         STAT_XY_RECT_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::XzRectHit => {
    //         STAT_XZ_RECT_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    //     RenderStat::YzRectHit => {
    //         STAT_YZ_RECT_HITS.fetch_add(1, Ordering::Relaxed);
    //     }
    // }
}
