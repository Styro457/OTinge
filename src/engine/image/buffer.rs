use crate::engine::image::color::Color;
use crate::engine::utils::math::pos2::Pos2;

pub mod rgba_u8_buffer;
pub mod rgba_f32_buffer;
pub mod r_u8_buffer;

pub trait ImageBuffer {

    fn new(size: Pos2) -> Self where Self: Sized;
    fn create(&self, size: Pos2) -> Box<dyn ImageBuffer>;
    fn get_wgpu_format(&self) -> wgpu::TextureFormat;
    fn get_bytes_per_pixel(&self) -> u32;
    fn get_data(&self) -> &[u8];
    fn set_pixel(&mut self, index: usize, color: Color);
    fn len(&self) -> usize;

}