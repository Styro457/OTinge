use crate::v1::engine::image::pixel::Pixel;
use crate::v1::engine::layer::Layer;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LayerData {
    blend_mode: u32,
    opacity: f32,
    mask_index: u32,
    texture_index: u32,
}

const MAX_LAYERS: usize = 1024;

pub fn layer_to_data<P: Pixel>(layer: &Layer<P>, texture_index: u32) -> LayerData {
    LayerData {
        blend_mode: layer.blend_mode as u32,
        opacity: layer.opacity,
        mask_index: layer.mask_index as u32,
        texture_index,
    }
}

pub fn layer_data_to_u32<P: Pixel>(layer: &Layer<P>) -> u32 {
    (layer.mask_index as u32) << 16
        | ((layer.opacity*255.0) as u32) << 8
        | (layer.blend_mode as u32)
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DocumentData {
    pub(crate) layer_count: u32,
    pub(crate) _padding: [u32; 3], // 16B alignment
}