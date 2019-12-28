#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub ray_trace_depth: u32,
    pub num_samples: u32,
}

impl RenderConfig {
    pub fn new(width: u32, height: u32, ray_trace_depth: u32, num_samples: u32) -> RenderConfig {
        RenderConfig {
            width,
            height,
            ray_trace_depth,
            num_samples,
        }
    }

    pub fn get_ray_trace_depth(&self) -> u32 {
        self.ray_trace_depth
    }
}
