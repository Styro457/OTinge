use wgpu::TextureFormat;
use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::color::Color;
use crate::engine::utils::math::pos2::Pos2;

pub struct RGBAf32Buffer {
    data: Vec<f32>,
}

impl ImageBuffer for RGBAf32Buffer {

    fn new(size: Pos2) -> Self {
        Self {
            data: vec![0.0; (size.x * size.y * 4) as usize],
        }
    }

    fn create(&self, size: Pos2) -> Box<dyn ImageBuffer> {
        Box::new(Self::new(size))
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

    fn set_pixel(&mut self, index: usize, color: Color) {
        let offset = index * 4;
        self.data[offset..(offset + 4)].copy_from_slice(&color.to_rgba_f32());
    }

    fn len(&self) -> usize {
        self.data.len()/4
    }
}