extern crate piston_window;
use chrono::prelude::*;
use piston_window::*;
use rtlib::render::{ImagePixelBuffer, RenderConfig};
use scenes::*;
use std::sync::{Arc, Mutex};

mod guirenderer;
use guirenderer::GuiRenderer;

fn get_datetime_file_marker() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y_%m_%d_%H_%M_%S").to_string()
}

fn main() {
    let width: u32 = 800;
    let height: u32 = 800;
    let depth: u32 = 50;
    let num_samples: u32 = 20;

    let mut window: PistonWindow = WindowSettings::new("rt-rs render", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let scene_generator = ManySpheresScene::new();

    let pixel_buffer = Arc::new(Mutex::new(ImagePixelBuffer::new(width, height)));
    let scene = scene_generator.create_scene();
    let camera = scene_generator.create_camera(width, height);

    let pb = pixel_buffer.clone();

    std::thread::spawn(move || {
        let render_config = RenderConfig::new(depth, num_samples, false);
        let renderer = Arc::new(GuiRenderer::new());
        renderer.render(
            pixel_buffer.clone(),
            Arc::new(Box::new(scene)),
            camera,
            &render_config,
        );

        std::fs::create_dir_all("./images").unwrap();
        let pixbuf = pixel_buffer.lock().unwrap();
        pixbuf.save_as_png(&format!(
            "./images/image_{}.png",
            get_datetime_file_marker()
        ));
    });

    while let Some(event) = window.next() {
        let pixbuf = pb.lock().unwrap();
        let texture = Texture::from_image(
            &mut window.create_texture_context(),
            pixbuf.get_image(),
            &TextureSettings::new(),
        )
        .unwrap();

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            image(&texture, context.transform, graphics);
            // rectangle(
            //     [1.0, 0.0, 0.0, 1.0], // red
            //     [0.0, 0.0, 100.0, 100.0],
            //     context.transform,
            //     graphics,
            // );
        });
    }
}
