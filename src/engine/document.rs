use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::tiles::manager::TileManager;
use crate::engine::layer::Layer;
use crate::engine::utils::math::pos2::Pos2;

pub struct Document {
    pub size: Pos2,
    pub base_layer: Layer,
    pub buffer_type: Box<dyn ImageBuffer>,
    pub tile_manager: TileManager,
}

impl Document {

    pub fn new(width: u32, height: u32, buffer_type: Box<dyn ImageBuffer>) -> Self {
        Self {
            size: Pos2::new(width, height),
            base_layer: Layer::new("Base Layer", None),
            buffer_type,
            tile_manager: TileManager::new(),
        }
    }

    pub fn layer_count(&self) -> u32 {
        self.base_layer.get_layer_count()
    }

}