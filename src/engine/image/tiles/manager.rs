use crate::engine::document::Document;
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::image::tiles::Tile;
use crate::engine::utils::math::pos2::Pos2;

pub struct TileManager {
    pub tiles: Vec<Tile>,
}

impl TileManager {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
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
            indirection: vec![u32::MAX; num_tiles],
        }
    }

    pub fn create_tile(&self, document: &Document) -> Tile {
        Tile {
            buffer: document.buffer_type.create(self.get_tile_size()),
        }
    }
}