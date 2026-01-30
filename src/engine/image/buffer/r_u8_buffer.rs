use image::DynamicImage;
use wgpu::TextureFormat;
use crate::engine::document::Document;
use crate::engine::image::buffer::{Color, ImageBuffer};
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::image::tiles::manager::TileManager;
use crate::engine::utils::math::pos2::Pos2;

pub struct Ru8Buffer {
    data: Vec<u8>,
}

impl Ru8Buffer {
    fn create_from_data(data: Vec<u8>) -> Box<dyn ImageBuffer> {
        Box::new(Self {
            data,
        })
    }
}

impl ImageBuffer for Ru8Buffer {

    fn new(size: Pos2) -> Self {
        Self {
            data: vec![0; (size.x * size.y) as usize],
        }
    }

    fn create(&self, size: Pos2) -> Box<dyn ImageBuffer> {
        Box::new(Self::new(size))
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

    fn convert_dynamic_image(&self, document: &mut Document, image: &DynamicImage) -> (Pos2, TileGrid) {
        todo!()
    }
}