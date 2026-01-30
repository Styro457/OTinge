use crate::engine::utils::slotmap::SlotMap;
use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::image::tiles::{Tile, EMPTY_TILE};
use crate::engine::utils::math::pos2::Pos2;

pub struct TileManager {
    pub tiles: SlotMap<Tile>,
    pub buffer_type: Box<dyn ImageBuffer>,
}

impl TileManager {
    pub fn new(buffer_type: Box<dyn ImageBuffer>) -> Self {
        Self {
            tiles: SlotMap::new(),
            buffer_type,
        }
    }

    pub fn get_tile_size(&self) -> Pos2 {
        Pos2::new(256, 256)
    }

    pub fn get_grid_size(&self, pixel_size: Pos2) -> Pos2 {
        let mut size = pixel_size / self.get_tile_size();
        if pixel_size.x % self.get_tile_size().x != 0 { size.x += 1; }
        if pixel_size.y % self.get_tile_size().y != 0 { size.y += 1; }
        size
    }

    pub fn create_tile_grid(&self, pixel_size: Pos2) -> TileGrid {
        let size = self.get_grid_size(pixel_size);

        let num_tiles = (size.x * size.y) as usize;

        TileGrid {
            size,
            indirection: vec![EMPTY_TILE; num_tiles],
            dirty_tiles: Vec::new(),
            dirty: true,
        }
    }

    fn create_tile(&mut self, tile_grid: &mut TileGrid, index: usize) {
        let tile: Tile = self.buffer_type.create(self.get_tile_size());
        let key = self.tiles.insert(tile);
        tile_grid.indirection[index] = key as u32;
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

    pub fn get_tile(&self, tile_grid: &TileGrid, x: u32, y: u32) -> Option<&Tile> {
        let tile_id = tile_grid.indirection[Self::get_index_from_pos(x, y)];
        if tile_id == EMPTY_TILE {
            return None
        }
        self.tiles.get(tile_id as usize)
    }

    pub fn get_tile_or_create(&mut self, tile_grid: &mut TileGrid, x: u32, y: u32) -> &mut Tile {
        let index = Self::get_index_from_pos(x, y);
        let tile_id = tile_grid.indirection[index];
        if tile_id == EMPTY_TILE {
            self.create_tile(tile_grid, index);
        }
        self.tiles.get_mut(tile_id as usize).unwrap()
    }

    pub fn get_tile_by_id(&self, id: u32) -> Option<&Tile> {
        self.tiles.get(id as usize)
    }

    pub fn get_index_from_pos(x: u32, y: u32) -> usize {
        ((x*y)+y) as usize
    }
}