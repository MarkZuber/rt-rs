use crate::render::Color;
use image::Pixel;

pub trait PixelBuffer: Sync {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn set_pixel_color(&mut self, x: u32, y: u32, color: Color);
    fn get_pixel_color(&self, x: u32, y: u32) -> Color;
}

#[derive(Debug)]
pub struct ImagePixelBuffer {
    imgbuf: image::RgbImage,
    is_y_up: bool,
}

impl ImagePixelBuffer {
    pub fn new(width: u32, height: u32) -> ImagePixelBuffer {
        ImagePixelBuffer {
            imgbuf: image::RgbImage::new(width, height),
            is_y_up: true,
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

    fn pixel_to_color(&self, pixel: &image::Rgb<u8>) -> Color {
        let channels = pixel.channels();
        Color::new(
            (channels[0] as f32) / 255.0,
            (channels[1] as f32) / 255.0,
            (channels[2] as f32) / 255.0,
        )
    }

    pub fn save_as_png(&self, output_file_path: &str) {
        self.imgbuf.save(output_file_path).unwrap();
    }

    fn f32_to_rgb(val: f32) -> u8 {
        (val * 255.0) as u8
    }

    #[inline]
    fn calculate_actual_y(&self, y: u32) -> u32 {
        if self.is_y_up {
            return self.get_height() - 1 - y;
        }
        y
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
        self.imgbuf.put_pixel(x, self.calculate_actual_y(y), pixel);
    }

    fn get_pixel_color(&self, x: u32, y: u32) -> Color {
        let pixel = self.imgbuf.get_pixel(x, y);
        self.pixel_to_color(pixel)
    }
}
