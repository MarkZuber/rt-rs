#[derive(Debug)]
pub struct RenderConfig {
    pub ray_trace_depth: u32,
    pub num_samples: u32,
    pub show_progress_bar: bool,
}

impl RenderConfig {
    pub fn new(ray_trace_depth: u32, num_samples: u32, show_progress_bar: bool) -> RenderConfig {
        RenderConfig {
            ray_trace_depth,
            num_samples,
            show_progress_bar,
        }
    }

    pub fn get_ray_trace_depth(&self) -> u32 {
        self.ray_trace_depth
    }
}
