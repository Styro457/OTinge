use crate::engine::utils::slotmap::SlotMap;
use crate::engine::document::Document;
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::image::tiles::Tile;
use crate::engine::utils::math::pos2::Pos2;

pub struct TileManager {
    pub tiles: SlotMap<Tile>,
}

impl TileManager {
    pub fn new() -> Self {
        Self {
            tiles: SlotMap::new()
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
            dirty_tiles: Vec::new(),
            dirty: true,
        }
    }

    pub fn create_tile(&self, document: &Document) -> Tile {
        Tile {
            buffer: document.buffer_type.create(self.get_tile_size()),
        }
    }

    pub fn add_tile(&mut self, tile_grid: &mut TileGrid, tile: Tile, position: Pos2) {
        let index = ((position.x * tile_grid.size.y) + position.y);
        let key = self.tiles.insert(tile);
        tile_grid.indirection[index as usize] = key as u32;
        tile_grid.dirty_tiles.push(index);
    }

    pub fn remove_tile(&mut self, tile_grid: &mut TileGrid, position: Pos2) {
        let index = ((position.x * tile_grid.size.y) + position.y) as usize;
        let key = tile_grid.indirection[index];
        tile_grid.indirection[index] = u32::MAX;
        self.tiles.remove(key as usize);
        tile_grid.dirty_tiles.push(index as u32);
    }

    pub fn get_tile(&self, id: u32) -> Option<&Tile> {
        self.tiles.get(id as usize)
    }
}