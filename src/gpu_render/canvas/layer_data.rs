use crate::engine::layer::Layer;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LayerData {
    blend_mode: u32,
    opacity: f32,
    _padding: [u32; 2], // 16B alignment
}

const MAX_LAYERS: usize = 1024;

pub fn layer_to_data (layer: &Layer) -> LayerData {
    LayerData {
        blend_mode: layer.blend_mode as u32,
        opacity: layer.opacity,
        _padding: [0; 2],
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DocumentData {
    pub(crate) layer_count: u32,
    pub(crate) _padding: [u32; 3], // 16B alignment
}