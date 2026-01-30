use crate::engine::utils::math::pos2::Pos2;

pub struct TileGrid {
    pub size: Pos2,
    pub indirection: Vec<u32>,

    pub dirty_tiles: Vec<u32>, // Indexes in indirection of dirty tiles
    pub dirty: bool // Whether the whole grid is dirty
}

impl TileGrid {
    pub fn get_data(&self) -> &[u8] {
        bytemuck::cast_slice(&self.indirection)
    }
}