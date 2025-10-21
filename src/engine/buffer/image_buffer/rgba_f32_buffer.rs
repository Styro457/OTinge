use wgpu::TextureFormat;
use crate::engine::buffer::image_buffer::ImageBuffer;

pub struct RGBAf32Buffer {
    data: Vec<f32>,
}

impl ImageBuffer for RGBAf32Buffer {

    fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size * 4],
        }
    }
    
    fn get_wgpu_format(&self) -> TextureFormat {
        TextureFormat::Rgba32Float
    }

    fn get_bytes_per_pixel(&self) -> u32 {
        16
    }

    fn get_data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.data)
    }
}