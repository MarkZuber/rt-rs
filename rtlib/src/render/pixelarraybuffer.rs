use super::color::Color;
use image::{Pixel, RgbaImage};

#[derive(Debug)]
pub struct PixelArrayBuffer {
    width: u32,
    height: u32,
    image_buffer: Vec<u8>,
    is_y_up: bool,
}

// TODO: clean up this class to share with pixelbuffer
impl PixelArrayBuffer {
    pub fn new(width: u32, height: u32) -> PixelArrayBuffer {
        PixelArrayBuffer {
            width,
            height,
            image_buffer: vec![0; (width as usize) * (height as usize) * 4],
            is_y_up: true,
        }
    }

    pub fn as_array(&self) -> &[u8] {
        &self.image_buffer
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn clamp_to_pixel(&self, color: Color) -> image::Rgba<u8> {
        let double_clamped = color.clamp();

        image::Rgba([
            PixelArrayBuffer::f32_to_rgb(double_clamped.r()),
            PixelArrayBuffer::f32_to_rgb(double_clamped.g()),
            PixelArrayBuffer::f32_to_rgb(double_clamped.b()),
            255,
        ])
    }

    fn pixel_to_color(&self, pixel: &image::Rgba<u8>) -> Color {
        let channels = pixel.channels();
        Color::new(
            (channels[0] as f32) / 255.0,
            (channels[1] as f32) / 255.0,
            (channels[2] as f32) / 255.0,
        )
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

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    fn calc_array_offset(&self, x: u32, y: u32) -> usize {
        (((self.calculate_actual_y(y) * self.width) + x) * 4) as usize
    }

    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        let pixel = self.clamp_to_pixel(color);
        let offset = self.calc_array_offset(x, y);
        self.image_buffer[offset] = pixel[0];
        self.image_buffer[offset + 1] = pixel[1];
        self.image_buffer[offset + 2] = pixel[2];
        self.image_buffer[offset + 3] = pixel[3];
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> Color {
        let rgba = self.get_pixel_rgba(x, y);
        self.pixel_to_color(&rgba)
    }

    fn get_pixel_rgba(&self, x: u32, y: u32) -> image::Rgba<u8> {
        let offset = self.calc_array_offset(x, y);
        image::Rgba([
            self.image_buffer[offset],
            self.image_buffer[offset + 1],
            self.image_buffer[offset + 2],
            self.image_buffer[offset + 3],
        ])
    }

    pub fn save_as_png(&self, output_file_path: &str) {
        let mut img = RgbaImage::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                img.put_pixel(x, y, self.get_pixel_rgba(x, y));
            }
        }
        img.save(output_file_path).unwrap();
    }
}
