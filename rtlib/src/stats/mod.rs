use num_format::{Locale, ToFormattedString};
use serde::Serialize;
use std::fmt;

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
        write!(
            f,
            "\nray_creates:          {}\n",
            self.ray_creates.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "camera_ray_creates:   {}\n",
            self.camera_ray_creates.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "aabb_hits:            {}\n",
            self.aabb_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "bvh_node_hits:        {}\n",
            self.bvh_node_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "cube_hits:            {}\n",
            self.cube_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "hitable_list_hits:    {}\n",
            self.hitable_list_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "medium_hits:          {}\n",
            self.medium_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "sphere_hits:          {}\n",
            self.sphere_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "triangle_hits:        {}\n",
            self.triangle_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "xy_rect_hits:         {}\n",
            self.xy_rect_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "xz_rect_hits:         {}\n",
            self.xz_rect_hits.to_formatted_string(&Locale::en)
        )?;
        write!(
            f,
            "yz_rect_hits:         {}\n",
            self.yz_rect_hits.to_formatted_string(&Locale::en)
        )
    }
}

impl RenderStats {
    pub fn new() -> RenderStats {
        RenderStats {
            ray_creates: 0,
            camera_ray_creates: 0,
            aabb_hits: 0,
            bvh_node_hits: 0,
            cube_hits: 0,
            hitable_list_hits: 0,
            medium_hits: 0,
            sphere_hits: 0,
            triangle_hits: 0,
            xy_rect_hits: 0,
            xz_rect_hits: 0,
            yz_rect_hits: 0,
        }
    }

    pub fn add(&self, other: RenderStats) -> RenderStats {
        RenderStats {
            ray_creates: self.ray_creates + other.ray_creates,
            camera_ray_creates: self.camera_ray_creates + other.camera_ray_creates,
            aabb_hits: self.aabb_hits + other.aabb_hits,
            bvh_node_hits: self.bvh_node_hits + other.bvh_node_hits,
            cube_hits: self.cube_hits + other.cube_hits,
            hitable_list_hits: self.hitable_list_hits + other.hitable_list_hits,
            medium_hits: self.medium_hits + other.medium_hits,
            sphere_hits: self.sphere_hits + other.sphere_hits,
            triangle_hits: self.triangle_hits + other.triangle_hits,
            xy_rect_hits: self.xy_rect_hits + other.xy_rect_hits,
            xz_rect_hits: self.xz_rect_hits + other.xz_rect_hits,
            yz_rect_hits: self.yz_rect_hits + other.yz_rect_hits,
        }
    }

    pub fn ray_create(&mut self) {
        self.ray_creates += 1;
    }
    pub fn camera_ray_create(&mut self) {
        self.camera_ray_creates += 1;
    }
    pub fn aabb_hit(&mut self) {
        self.aabb_hits += 1;
    }
    pub fn bvh_node_hit(&mut self) {
        self.bvh_node_hits += 1;
    }
    pub fn cube_hit(&mut self) {
        self.cube_hits += 1;
    }
    pub fn hitable_list_hit(&mut self) {
        self.hitable_list_hits += 1;
    }
    pub fn medium_hit(&mut self) {
        self.medium_hits += 1;
    }
    pub fn sphere_hit(&mut self) {
        self.sphere_hits += 1;
    }
    pub fn triangle_hit(&mut self) {
        self.triangle_hits += 1;
    }
    pub fn xy_rect_hit(&mut self) {
        self.xy_rect_hits += 1;
    }
    pub fn xz_rect_hit(&mut self) {
        self.xz_rect_hits += 1;
    }
    pub fn yz_rect_hit(&mut self) {
        self.yz_rect_hits += 1;
    }
}

// impl fmt::Display for RenderStats {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let j = serde_json::to_string_pretty(&self).unwrap();
//         write!(f, "{}", j)
//     }
// }
