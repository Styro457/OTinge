use wgpu::TextureFormat;
use crate::engine::buffer::image_buffer::ImageBuffer;

pub struct RGBAu8Buffer {
    data: Vec<u8>,
}

impl ImageBuffer for RGBAu8Buffer {

    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size * 4],
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
}