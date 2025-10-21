use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::tiles::{Tile, TileGrid};
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

    pub fn get_tile_size(&self) -> Pos2 {
        Pos2::new(64, 64)
    }

    pub fn create_tile_grid(&self, pixel_size: Pos2) -> TileGrid {
        let mut size = pixel_size / self.get_tile_size();
        if pixel_size.x % self.get_tile_size().x != 0 { size.x += 1; }
        if pixel_size.y % self.get_tile_size().y != 0 { size.y += 1; }

        let num_tiles = (size.x * size.y) as usize;

        TileGrid {
            size,
            tiles: Vec::new(),
            indirection: vec![u16::MAX; num_tiles],
        }
    }

    pub fn create_tile(&self) -> Tile {
        Tile {
            buffer: self.buffer_type.create(self.get_tile_size()),
        }
    }

}