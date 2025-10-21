use wgpu::TextureFormat;
use crate::engine::image::buffer::{Color, ImageBuffer};
use crate::engine::utils::math::pos2::Pos2;

pub struct Ru8Buffer {
    data: Vec<u8>,
}

impl ImageBuffer for Ru8Buffer {

    fn new(size: Pos2) -> Self {
        Self {
            data: vec![0; (size.x * size.y) as usize],
        }
    }

    fn get_wgpu_format(&self) -> TextureFormat {
        TextureFormat::R8Unorm
    }

    fn get_bytes_per_pixel(&self) -> u32 {
        1
    }

    fn get_data(&self) -> &[u8] {
        &*self.data
    }

    fn set_pixel(&mut self, index: usize, color: Color) {
        self.data[index] = color.to_r_u8();
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}