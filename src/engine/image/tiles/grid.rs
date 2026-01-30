use bitvec::vec::BitVec;
use slotmap::Key;
use crate::engine::image::tiles::manager::{TileKey, TileManager};
use crate::engine::image::tiles::Tile;
use crate::engine::utils::math::pos2::Pos2;

pub struct TileGrid {
    pub size: Pos2,
    pub indirection: Vec<TileKey>,
    pub dirty_tiles: BitVec,
}

impl TileGrid {
    pub fn add_tile(&mut self, tile: Tile, position: Pos2, tile_manager: &mut TileManager) {
        let index = ((position.x * self.size.y) + position.y) as usize;
        let key = tile_manager.tiles.insert(tile);
        self.indirection[index] = key;
        self.dirty_tiles.set(index, true);
    }

    pub fn remove_tile(&mut self, position: Pos2, tile_manager: &mut TileManager) {
        let index = ((position.x * self.size.y) + position.y) as usize;
        let key = self.indirection[index];
        self.indirection[index] = TileKey::null();
        tile_manager.tiles.remove(key);
        self.dirty_tiles.set(index, true);
    }
}