use glam::{Mat4, Quat, Vec3};
use crate::engine::layer::Layer;
use crate::engine::utils::math::pos2::Pos2;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LayerData {
    blend_mode: u32,
    opacity: f32,
    size: [f32; 2],
    transform: [f32; 16],
    //_padding: [u32; 3], // 16B alignment
}

const MAX_LAYERS: usize = 1024;

pub fn layer_to_data (layer: &Layer) -> LayerData {

    let size = layer.get_size();

    let transform = if(size != Pos2::ZERO) {
        let center = size.to_vec2() / 2.0;

        // Build the "Forward" matrix (Model -> World)
        // Order: Translate(Pos) * Rotate(Rot) * Scale(Scale) * Translate(-Center)
        let model_matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(layer.transform.scale.x, layer.transform.scale.y, 1.0),
            Quat::from_rotation_z(layer.transform.rotation),
            Vec3::new(layer.transform.position.x as f32, layer.transform.position.y as f32, 0.0),
        ) * Mat4::from_translation(Vec3::new(-center.x, -center.y, 0.0));

        // Calculate the "Inverse" matrix (World -> Model)
        let inverse_matrix = model_matrix.inverse();
        inverse_matrix.to_cols_array()
    }
    else {
        [0.0; 16]
    };

    LayerData {
        blend_mode: layer.blend_mode as u32,
        opacity: layer.opacity,
        size: layer.get_size().into(),
        transform,
        //_padding: [0; 3],
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DocumentData {
    pub(crate) layer_count: u32,
    pub(crate) _padding: [u32; 3], // 16B alignment
}