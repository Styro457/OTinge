use crate::engine::utils::math::pos2::Pos2;
use crate::engine::image::buffer::ImageBuffer;

pub struct Tile {
    pub buffer: Box<dyn ImageBuffer>,
}

pub struct TileGrid {
    pub size: Pos2,
    pub tiles: Vec<Tile>,
    pub indirection: Vec<u16>
}

impl TileGrid {
    pub fn add_tile(&mut self, tile: Tile, position: Pos2) {
        let index = self.tiles.len();
        self.tiles.push(tile);
        self.indirection[((position.x * self.size.y) + position.y) as usize] = index as u16;
    }
}