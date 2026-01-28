use crate::engine::document::Document;
use crate::engine::image::color::Color;
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::image::tiles::Tile;
use crate::engine::painters::Painter;
use crate::engine::utils::math::pos2::Pos2;


//This is a temporary painter for testing purposes. Final Solid implementation should be an effect.
pub struct SolidPainter {
    size: Pos2,
    color: Color,
    tile_grid: TileGrid,
}

impl SolidPainter {

    pub fn new(document: &Document, size: Pos2, color: Color) -> Self {
        let tile_grid = document.tile_manager.create_tile_grid(size);
        let mut obj = Self {
            size,
            color,
            tile_grid,
        };
        obj.set_color(color, &document);
        obj
    }

    pub fn set_color(&mut self, color: Color, document: &Document) {
        self.color = color;
        let tile_grid = &mut self.tile_grid;
        for i in 0..tile_grid.size.x {
            for j in 0..tile_grid.size.y {
                let mut tile: Tile = document.tile_manager.create_tile(document);
                for x in 0..tile.buffer.len() {
                    tile.buffer.set_pixel(x, self.color);
                }
                tile_grid.add_tile(tile, Pos2::new(i, j));
            }
        }
    }
}

impl Painter for SolidPainter {

    fn get_tilegrid(&self) -> &TileGrid {
        &self.tile_grid
    }

}