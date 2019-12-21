use crate::render::{Color, PixelBuffer};
use crate::textures::Texture;
use crate::{Point2, Vector3};
use std::sync::Arc;

pub struct ImageTexture {
    pixel_buffer: Arc<Box<dyn PixelBuffer + Send>>,
}

impl ImageTexture {
    pub fn new(pixel_buffer: Arc<Box<dyn PixelBuffer + Send>>) -> Arc<Box<ImageTexture>> {
        Arc::new(Box::new(ImageTexture { pixel_buffer }))
    }
}

fn clamp_i32_to_u32(val: i32, min: i32, max: i32) -> u32 {
    let mut final_val = if val < min { min } else { val };
    final_val = if final_val > max { max } else { final_val };
    final_val as u32
}

impl Texture for ImageTexture {
    fn get_value(&self, uv_coords: Point2<f32>, _p: Vector3<f32>) -> Color {
        let width = self.pixel_buffer.get_width() as i32;
        let height = self.pixel_buffer.get_height() as i32;

        let i = clamp_i32_to_u32((uv_coords.x * (width as f32)) as i32, 0, width - 1);
        let j = clamp_i32_to_u32(
            ((1.0 - uv_coords.y * (height as f32)) - 0.001) as i32,
            0,
            height - 1,
        );

        self.pixel_buffer.get_pixel_color(i, j)
    }
}
