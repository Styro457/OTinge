use wgpu::TextureFormat;
use crate::engine::image::buffer::ImageBuffer;

pub struct Ru8Buffer {
    data: Vec<u8>,
}

impl ImageBuffer for Ru8Buffer {
    
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
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
}