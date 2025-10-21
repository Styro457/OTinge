use crate::engine::image::tile::Tile;
use crate::engine::utils::math::pos2::Pos2;

pub struct TileGrid {
    pub size: Pos2,
    pub tiles: Vec<Tile>,
    pub indirection: Vec<u16>
}

impl TileGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let size = Pos2::new(width, height);
        let num_tiles = (width * height) as usize;
        Self {
            size,
            tiles: Vec::new(),
            indirection: vec![u16::MAX; num_tiles],
        }
    }
}