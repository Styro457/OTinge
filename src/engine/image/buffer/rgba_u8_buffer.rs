use wgpu::TextureFormat;
use crate::engine::image::buffer::{ImageBuffer};
use crate::engine::image::color::Color;
use crate::engine::utils::math::pos2::Pos2;

pub struct RGBAu8Buffer {
    data: Vec<u8>,
}

impl ImageBuffer for RGBAu8Buffer {

    fn new(size: Pos2) -> Self {
        Self {
            data: vec![0; (size.x * size.y * 4) as usize],
        }
    }

    fn get_wgpu_format(&self) -> TextureFormat {
        TextureFormat::Rgba8UnormSrgb
    }

    fn get_bytes_per_pixel(&self) -> u32 {
        4
    }

    fn get_data(&self) -> &[u8] {
        &*self.data
    }

    fn set_pixel(&mut self, index: usize, color: Color) {
        let offset = index * 4;
        self.data[offset..(offset + 4)].copy_from_slice(&color.to_rgba_u8());
    }

    fn len(&self) -> usize {
        self.data.len()/4
    }
}