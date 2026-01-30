use crate::engine::document::Document;
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::painters::Painter;
use crate::engine::utils::math::pos2::Pos2;

pub struct ImagePainter {
    size: Pos2,
    tile_grid: TileGrid,
}

impl ImagePainter {
    pub fn new(document: &mut Document, path: &str, ) -> Self {
        let img = image::open(path).unwrap();

        // TODO: Find a way to do this without creating a new buffer type
        let buffer_type = document.tile_manager.buffer_type.create(Pos2::ZERO);
        let (size, tile_grid) = buffer_type.convert_dynamic_image(document, &img);
        Self {
            size,
            tile_grid,
        }
    }
}

impl Painter for ImagePainter {

    fn get_tilegrid(&self) -> &TileGrid {
        &self.tile_grid
    }

    fn get_size(&self) -> Pos2 {
        self.size
    }

}