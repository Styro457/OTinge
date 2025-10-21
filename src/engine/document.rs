use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::tiles::TileGrid;
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
    
    pub fn create_tile_grid(&self, width: u32, height: u32) -> TileGrid {
        let size = Pos2::new(width, height);
        let num_tiles = (width * height) as usize;
        TileGrid {
            size,
            tiles: Vec::new(),
            indirection: vec![u16::MAX; num_tiles],
        }
    }

}