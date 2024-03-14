use glow::{HasContext, NativeTexture};
use imgui::{Condition, TextureId};
use std::sync::mpsc::{self, Receiver};
use winit::dpi::PhysicalSize;

use rtlib::render::{Pixel, PixelArrayBuffer};

pub struct TextureUi {
    texture_id: TextureId,
    native_texture: NativeTexture,
    pixel_buffer: PixelArrayBuffer,
    pixel_rx: Receiver<Pixel>,
    width: u32,
    height: u32,

    pixel_render_counter: u32,
}

impl TextureUi {
    pub fn new(
        gl: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
        width: u32,
        height: u32,
        pixel_rx: Receiver<Pixel>,
    ) -> Self {
        let native_texture: glow::Texture =
            unsafe { gl.create_texture().expect("Cannot create texture") };

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(native_texture));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
        }

        let texture_id = textures.insert(native_texture);
        let pixel_buffer = PixelArrayBuffer::new(width, height);

        TextureUi {
            texture_id,
            native_texture,
            pixel_buffer,
            width,
            height,
            pixel_rx,
            pixel_render_counter: 10,
        }
    }

    pub fn update_texture(&mut self, gl: &glow::Context) {
        // read pixels from the channel

        self.pixel_render_counter -= 1;
        if self.pixel_render_counter <= 0 {
            loop {
                match self.pixel_rx.try_recv() {
                    Ok(pixel) => {
                        self.pixel_buffer
                            .set_pixel_color(pixel.x, pixel.y, pixel.color);
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                }
            }
            self.pixel_render_counter = 10;
        }

        // Update the GPU texture with the modified pixel data
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.native_texture));
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(self.pixel_buffer.as_array()),
            );
        }
    }
    pub fn show(&self, ui: &imgui::Ui, physical_size: PhysicalSize<u32>) {
        let img_width = ((physical_size.width / 2) - 300) as f32; // todo: make 300 const (300 is width of imgui config window)
        let img_height = (physical_size.height / 2) as f32;

        ui.window("Render Window")
            .position([0.0, 0.0], Condition::FirstUseEver)
            .size([img_width, img_height], Condition::FirstUseEver)
            .build(|| {
                imgui::Image::new(self.texture_id, [img_width - 30.0, img_height - 40.0]).build(ui);
            });
    }
}
