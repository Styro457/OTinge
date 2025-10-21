use crate::engine::image::buffer::ImageBuffer;
use crate::engine::layer::Layer;
use crate::engine::utils::math::pos2::Pos2;

pub struct Document {
    pub size: Pos2,
    pub base_layer: Layer,
    pub buffer_type: Box<dyn ImageBuffer>,
}

impl Document {

    pub fn new(width: u32, height: u32, buffer_type: Box<dyn ImageBuffer>) -> Self {
        Self {
            size: Pos2::new(width, height),
            base_layer: Layer::new("Base Layer", None),
            buffer_type,
        }
    }

}