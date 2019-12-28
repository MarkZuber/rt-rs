#![feature(test)]

extern crate piston_window;
use chrono::prelude::*;
use piston_window::*;
use rtlib::render::{
    ConsoleRenderer, NffParser, PixelBuffer, RenderConfig, Renderer, SceneGenerator,
};
use scenes::*;
use std::sync::{Arc, Mutex};
mod guirenderer;
use guirenderer::GuiRenderer;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum SceneType {
    // Something on spheres.
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

    #[structopt(short = "w", long = "width", default_value = "300")]
    image_width: u32,

    #[structopt(short = "h", long = "height", default_value = "300")]
    image_height: u32,

    #[structopt(short = "l", long = "logging")]
    logging: bool,

    #[structopt(subcommand)]
    scene_type: SceneType,

    #[structopt(short = "s", long = "samples", default_value = "100")]
    numsamples: u32,

    #[structopt(short = "d", long = "depth", default_value = "50")]
    depth: u32,

    #[structopt(short = "n", long = "no-save")]
    no_save: bool,
}

fn get_datetime_file_marker() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y_%m_%d_%H_%M_%S").to_string()
}

fn do_render(
    renderer: Arc<Box<dyn Renderer>>,
    should_save: bool,
    render_config: &RenderConfig,
    pixel_buffer: Arc<Mutex<PixelBuffer>>,
    scene_generator: Arc<Box<dyn SceneGenerator + Send>>,
) {
    let pb = pixel_buffer.clone();
    let scene = scene_generator.get_scene();
    let camera = scene_generator.get_camera();

    renderer.render(
        pixel_buffer,
        Arc::new(Box::new(scene)),
        camera,
        render_config,
    );

    if should_save {
        std::fs::create_dir_all("./images").unwrap();
        let pixbuf = pb.lock().unwrap();
        pixbuf.save_as_png(&format!(
            "./images/image_{}.png",
            get_datetime_file_marker()
        ));
    }
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

    let render_config = Arc::new(scene_generator.get_render_config());

    let pixel_buffer = Arc::new(Mutex::new(PixelBuffer::new(
        render_config.width,
        render_config.height,
    )));

    if opts.console {
        let renderer = Arc::new(ConsoleRenderer::new(true));
        do_render(
            renderer,
            !opts.no_save,
            &render_config,
            pixel_buffer.clone(),
            scene_generator,
        );
    } else {
        let pb = pixel_buffer.clone();

        let mut window: PistonWindow =
            WindowSettings::new("rt-rs render", [render_config.width, render_config.height])
                .exit_on_esc(true)
                .build()
                .unwrap();

        let scg = scene_generator.clone();
        let should_save = !opts.no_save;
        std::thread::spawn(move || {
            let renderer = Arc::new(GuiRenderer::new());
            do_render(
                renderer,
                should_save,
                &render_config,
                pixel_buffer.clone(),
                scg,
            );
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
            });
        }
    };
}

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

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use rtlib::render::RenderExec;
    use test::Bencher;

    fn create_bench_scene() -> RenderExec {
        RenderExec::new(
            ManySpheresScene::new(&RenderConfig::new(1, 1, 1, 20)),
            ConsoleRenderer::new(false),
        )
    }

    #[bench]
    fn bench_render(b: &mut Bencher) {
        let mut render_exec = create_bench_scene();
        b.iter(|| {
            render_exec.execute();
        });
    }
}
