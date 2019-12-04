use crate::render::Color;

pub trait PixelBuffer: Sync {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn set_pixel_color(&mut self, x: u32, y: u32, color: Color);
}

#[derive(Debug)]
pub struct ImagePixelBuffer {
    imgbuf: image::RgbImage,
}

impl ImagePixelBuffer {
    pub fn new(width: u32, height: u32) -> ImagePixelBuffer {
        ImagePixelBuffer {
            imgbuf: image::RgbImage::new(width, height),
        }
    }
}

impl ImagePixelBuffer {
    fn clamp_to_pixel(&self, color: Color) -> image::Rgb<u8> {
        let double_clamped = color.clamp();

        image::Rgb([
            ImagePixelBuffer::f32_to_rgb(double_clamped.r()),
            ImagePixelBuffer::f32_to_rgb(double_clamped.g()),
            ImagePixelBuffer::f32_to_rgb(double_clamped.b()),
        ])
    }

    pub fn save_as_png(&self, output_file_path: &str) {
        self.imgbuf.save(output_file_path).unwrap();
    }

    fn f32_to_rgb(val: f32) -> u8 {
        (val * 255.0) as u8
    }
}

impl PixelBuffer for ImagePixelBuffer {
    fn get_width(&self) -> u32 {
        self.imgbuf.width()
    }

    fn get_height(&self) -> u32 {
        self.imgbuf.height()
    }

    fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        let pixel = self.clamp_to_pixel(color);
        self.imgbuf.put_pixel(x, y, pixel);
    }
}
