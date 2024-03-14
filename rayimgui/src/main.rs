#![allow(dead_code)]

//! Example showing the same functionality as
//! `imgui-examples/examples/custom_textures.rs`
//!
//! Not that the texture uses the internal format `glow::SRGB`, so that
//! OpenGL automatically converts colors to linear space before the shaders.
//! The renderer assumes you set this internal format correctly like this.

extern crate glow;
extern crate glutin;
use chrono::prelude::*;
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use std::{
    num::NonZeroU32,
    sync::{mpsc::channel, Arc},
    time::Instant,
};

use glow::HasContext;
use glutin::surface::GlSurface;
use imgui::Condition;

use imgui_glow_renderer::Renderer;
use rtlib::render::{NffParser, Pixel, RenderConfig};
use scenes::{CornellBoxScene, ManySpheresScene};
use structopt::StructOpt;
use winit::event_loop::ControlFlow;

mod utils;
pub use cgmath::{vec3, InnerSpace, Point2, Vector3};
use utils::{imguirenderer::ImguiRenderer, textureui::TextureUi};

#[derive(Debug, StructOpt)]
enum SceneType {
    #[structopt(about = "Many Spheres Scene")]
    Spheres,
    #[structopt(about = "Cornell Box Scene")]
    CornellBox,
    #[structopt(about = "Parse Nff File")]
    Nff {
        #[structopt(help = "Name of file in nff/ subdirectory")]
        filename: String,
    },
}

#[derive(Debug, StructOpt)]
struct MainOptions {
    #[structopt(short = "c", long = "console")]
    console: bool,

    #[structopt(short = "w", long = "width", default_value = "900")]
    image_width: u32,

    #[structopt(short = "h", long = "height", default_value = "900")]
    image_height: u32,

    #[structopt(short = "l", long = "logging")]
    logging: bool,

    #[structopt(subcommand)]
    scene_type: SceneType,

    #[structopt(short = "s", long = "samples", default_value = "50")]
    numsamples: u32,

    #[structopt(short = "d", long = "depth", default_value = "8")]
    depth: u32,

    #[structopt(short = "n", long = "no-save")]
    no_save: bool,
    // #[structopt(short = "p", long = "profile")]
    // profile: bool,
}

fn get_datetime_file_marker() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y_%m_%d_%H_%M_%S").to_string()
}

fn main() {
    let opts = MainOptions::from_args();
    enable_logging(&opts);

    let render_config = RenderConfig::new(
        opts.image_width,
        opts.image_height,
        opts.depth,
        opts.numsamples,
    );

    let scene_generator = match opts.scene_type {
        SceneType::CornellBox => CornellBoxScene::new(&render_config),
        SceneType::Spheres => ManySpheresScene::new(&render_config),
        SceneType::Nff { filename } => {
            std::fs::create_dir_all("./nff").unwrap();
            let nff_file_path = format!("./nff/{}.nff", filename);
            NffParser::new(&nff_file_path, &render_config)
        }
    };

    // TODO: let render_config height/width control more in the UI
    let render_config = Arc::new(scene_generator.get_render_config());

    let (event_loop, window, surface, context) =
        utils::window::create_window("Custom textures", None);
    let (mut winit_platform, mut imgui_context) = utils::window::imgui_init(&window);
    let gl = utils::window::glow_context(&context);
    // This time, we tell OpenGL this is an sRGB framebuffer and OpenGL will
    // do the conversion to sSGB space for us after the fragment shader.
    unsafe { gl.enable(glow::FRAMEBUFFER_SRGB) };

    let mut textures = imgui::Textures::<glow::Texture>::default();
    // Note that `output_srgb` is `false`. This is because we set
    // `glow::FRAMEBUFFER_SRGB` so we don't have to manually do the conversion
    // in the shader.
    let mut ig_renderer = Renderer::initialize(&gl, &mut imgui_context, &mut textures, false)
        .expect("failed to create renderer");

    // TODO: need to get dimensions properly figured out across the board
    let width: u32 = 500;
    let height: u32 = 500;

    // TODO: create the send/recv channels here, send the receiver into TextureUi and the sender into RayTraceRenderer (so it can spawn threads to send pixel data)
    let mut is_red: bool = true;

    let (send, recv) = channel::<Pixel>();
    let renderer = ImguiRenderer::new(width, height, send, true);
    let mut tracer_ui = TextureUi::new(&gl, &mut textures, width, height, recv);

    renderer.start_ex(render_config, scene_generator);

    let mut last_frame = Instant::now();

    event_loop
        .run(move |event, window_target| {
            // Note we can potentially make the loop more efficient by
            // changing the `Poll` (default) value to `ControlFlow::Wait`
            // but be careful to test on all target platforms!
            window_target.set_control_flow(ControlFlow::Poll);

            match event {
                winit::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui_context
                        .io_mut()
                        .update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                winit::event::Event::AboutToWait => {
                    winit_platform
                        .prepare_frame(imgui_context.io_mut(), &window)
                        .unwrap();

                    window.request_redraw();
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::RedrawRequested,
                    ..
                } => {
                    unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };

                    let delta_time = imgui_context.io().delta_time;
                    let ui = imgui_context.frame();

                    // TODO: improve this to get content inner size
                    // and include constants for side panel size
                    let pos = window.inner_size();
                    let winxpos = (pos.width / 2) as f32 - 300.0;
                    let winyheight = (pos.height / 2) as f32;

                    // add in imgui config window
                    ui.window("Configuration")
                        .position([winxpos, 0.0], Condition::FirstUseEver)
                        .size([300.0, winyheight], Condition::FirstUseEver)
                        .build(|| {
                            if ui.button("Render") {
                                is_red = !is_red;
                            }
                            ui.text(format!("FPS: ({})", 1.0 / delta_time));
                        });

                    tracer_ui.update_texture(&gl);
                    tracer_ui.show(&ui, window.inner_size());

                    winit_platform.prepare_render(ui, &window);
                    let draw_data = imgui_context.render();
                    ig_renderer
                        .render(&gl, &textures, draw_data)
                        .expect("error rendering imgui");

                    surface
                        .swap_buffers(&context)
                        .expect("Failed to swap buffers");
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::Resized(new_size),
                    ..
                } => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface.resize(
                            &context,
                            NonZeroU32::new(new_size.width).unwrap(),
                            NonZeroU32::new(new_size.height).unwrap(),
                        );
                    }
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    window_target.exit();
                }
                winit::event::Event::LoopExiting => {
                    ig_renderer.destroy(&gl);
                }
                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                }
            }
        })
        .expect("EventLoop error");
}

// TODO: consolidate this with other usage (move to utils)
fn enable_logging(opts: &MainOptions) {
    if opts.logging {
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build("log/output.log")
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
}
