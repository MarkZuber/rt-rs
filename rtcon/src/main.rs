use chrono::prelude::*;
use rtlib::render::{ImagePixelBuffer, PerPixelRenderer, RenderConfig};
use rtlib::{vec3, Vector3};
use scenes::*;
use std::sync::Arc;

fn get_datetime_file_marker() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y_%m_%d_%H_%M_%S").to_string()
}

fn main() {
    let ray_trace_depth = 50;
    let num_samples = 300;

    let image_width = 500;
    let image_height = 500;

    let render_config = RenderConfig::new(ray_trace_depth, num_samples);
    let renderer = PerPixelRenderer::new();
    let mut pixel_buffer = ImagePixelBuffer::new(image_width, image_height);

    let scene = create_cornell_box_scene();
    let camera = create_cornell_box_camera(image_width, image_height);

    let _renderer_data = renderer.render(
        &mut pixel_buffer,
        Arc::new(Box::new(scene)),
        camera,
        &render_config,
    );

    std::fs::create_dir_all("./images").unwrap();

    pixel_buffer.save_as_png(&format!(
        "./images/image_{}.png",
        get_datetime_file_marker()
    ));
}
