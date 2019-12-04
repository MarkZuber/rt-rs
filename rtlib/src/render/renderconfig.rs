#[derive(Debug)]
pub struct RenderConfig {
    pub ray_trace_depth: u32,
    pub num_samples: u32,
}

impl RenderConfig {
    pub fn new(ray_trace_depth: u32, num_samples: u32) -> RenderConfig {
        RenderConfig {
            ray_trace_depth,
            num_samples,
        }
    }

    pub fn get_ray_trace_depth(&self) -> u32 {
        self.ray_trace_depth
    }
}
