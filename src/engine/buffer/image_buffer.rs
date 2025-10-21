mod rgba_u8_buffer;
mod rgba_f32_buffer;
mod r_u8_buffer;

pub trait ImageBuffer {

    fn new(size: usize) -> Self where Self: Sized;
    fn get_wgpu_format(&self) -> wgpu::TextureFormat;
    fn get_bytes_per_pixel(&self) -> u32;
    fn get_data(&self) -> &[u8];
}