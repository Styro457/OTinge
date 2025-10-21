use crate::engine::utils::math::pos2::Pos2;
use crate::engine::image::buffer::ImageBuffer;

pub struct Tile {
    pub pos: Pos2,
    pub buffer: Box<dyn ImageBuffer>,
}

pub struct TileGrid {
    pub size: Pos2,
    pub tiles: Vec<Tile>,
    pub indirection: Vec<u16>
}