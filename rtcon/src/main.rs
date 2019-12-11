// #![feature(test)]

use chrono::prelude::*;
use rtlib::render::RenderExec;
use scenes::*;

fn get_datetime_file_marker() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y_%m_%d_%H_%M_%S").to_string()
}

fn main() {
    let mut render_exec = RenderExec::new(CornellBoxScene::new(), 300, 300, 50, 30, false, true);
    render_exec.execute();

    std::fs::create_dir_all("./images").unwrap();
    render_exec.save_pixel_buffer(&format!(
        "./images/image_{}.png",
        get_datetime_file_marker()
    ));
}

// extern crate test;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     fn create_bench_scene(use_bvh: bool) -> RenderExec {
//         RenderExec::new(CornellBoxScene::new(), 50, 50, 10, 30, use_bvh, false)
//     }

//     #[bench]
//     fn bench_with_bvh(b: &mut Bencher) {
//         let mut render_exec = create_bench_scene(true);
//         b.iter(|| {
//             render_exec.execute();
//         });
//     }

//     #[bench]
//     fn bench_no_bvh(b: &mut Bencher) {
//         let mut render_exec = create_bench_scene(false);
//         b.iter(|| {
//             render_exec.execute();
//         });
//     }
// }
