pub mod grid;
pub mod manager;
use crate::engine::image::buffer::ImageBuffer;

pub const TILE_SIZE: u32 = 256;
pub const EMPTY_TILE: u32 = u32::MAX;

pub type Tile = Box<dyn ImageBuffer>;