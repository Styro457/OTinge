use crate::engine::image::tiles::grid::TileGrid;

pub mod image;
pub mod text;
pub mod solid;

pub trait Painter {

    fn get_tilegrid(&self) -> &TileGrid;

}