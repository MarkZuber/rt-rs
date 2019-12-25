#![feature(test)]

// #[macro_use]
extern crate log;
extern crate log4rs;

use chrono::prelude::*;
use rtlib::render::RenderExec;
use scenes::*;
use structopt::StructOpt;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn get_datetime_file_marker() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y_%m_%d_%H_%M_%S").to_string()
}

#[derive(Debug, StructOpt)]
struct MainOptions {
    #[structopt(long, short)]
    logging: bool,
}

fn main() {
    let opts = MainOptions::from_args();

    enable_logging(&opts);

    let scene = ManySpheresScene::new();
    // let scene = CornellBoxScene::new();

    let mut render_exec = RenderExec::new(scene, 3072, 1920, 50, 2000, true);
    render_exec.execute();

    std::fs::create_dir_all("./images").unwrap();
    render_exec.save_pixel_buffer(&format!(
        "./images/image_{}.png",
        get_datetime_file_marker()
    ));
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
    use test::Bencher;

    fn create_bench_scene() -> RenderExec {
        RenderExec::new(ManySpheresScene::new(), 1, 1, 1, 20, false)
    }

    #[bench]
    fn bench_render(b: &mut Bencher) {
        let mut render_exec = create_bench_scene();
        b.iter(|| {
            render_exec.execute();
        });
    }
}
