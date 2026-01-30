use std::cmp::min;
use crate::engine::document::Document;
use crate::engine::image::color::Color;
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::painters::Painter;
use crate::engine::utils::math::pos2::Pos2;


//This is a temporary painter for testing purposes. Final Solid implementation should be an effect.
pub struct SolidPainter {
    size: Pos2,
    color: Color,
    tile_grid: TileGrid,
}

impl SolidPainter {

    pub fn new(document: &mut Document, size: Pos2, color: Color) -> Self {
        let tile_grid = document.tile_manager.create_tile_grid(document.size);
        let mut obj = Self {
            size,
            color,
            tile_grid,
        };
        obj.set_color(color, document);
        obj
    }

    // Not optimized implementation since this Painter is only intended for testing
    pub fn set_color(&mut self, color: Color, document: &mut Document) {
        self.color = color;

        let tile_size = document.tile_manager.get_tile_size();

        let tiles_x = (self.size.x + tile_size.x - 1) / tile_size.x;
        let tiles_y = (self.size.y + tile_size.y - 1) / tile_size.y;

        let tile_grid = &mut self.tile_grid;

        for i in 0..tiles_x {
            for j in 0..tiles_y {
                let tile = document.tile_manager.get_tile_or_create(tile_grid, i, j);

                let tile_start_x = i * tile_size.x;
                let tile_start_y = j * tile_size.y;

                let valid_width = min(tile_size.x, self.size.x.saturating_sub(tile_start_x));
                let valid_height = min(tile_size.y, self.size.y.saturating_sub(tile_start_y));

                for y in 0..valid_width {
                    for x in 0..valid_height {
                        let index = (y * tile_size.x) + x;
                        tile.set_pixel(index as usize, self.color);
                    }
                }
            }
        }
        tile_grid.dirty = true;
    }
}

impl Painter for SolidPainter {

    fn get_tilegrid(&self) -> &TileGrid {
        &self.tile_grid
    }

    fn get_size(&self) -> Pos2 {
        self.size
    }

}