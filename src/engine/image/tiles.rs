pub mod grid;
pub mod manager;

use crate::engine::image::buffer::ImageBuffer;

pub struct Tile {
    pub buffer: Box<dyn ImageBuffer>,
}