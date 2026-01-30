use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::utils::math::pos2::Pos2;

pub mod image;
pub mod text;
pub mod solid;

pub trait Painter {

    fn get_tilegrid(&self) -> &TileGrid;

    fn get_size(&self) -> Pos2;

}