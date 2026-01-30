// use crate::engine::document::Document;
// use crate::engine::image::color::Color;
// use crate::engine::image::tiles::Tile;
// use crate::engine::image::tiles::grid::TileGrid;
// use crate::engine::painters::Painter;
// use crate::engine::utils::math::pos2::Pos2;
// 
// struct ImagePainter {
//     size: Pos2,
//     tile_grid: TileGrid,
// }
// 
// impl ImagePainter {
// 
//     pub fn new(document: &mut Document, size: Pos2) -> Self {
//         let mut tile_grid = document.tile_manager.create_tile_grid(size);
// 
//         for i in 0..tile_grid.size.x {
//             for j in 0..tile_grid.size.y {
//                 let mut tile: Tile = document.tile_manager.create_tile(document);
//                 for x in 0..tile.buffer.len() {
//                     tile.buffer.set_pixel(x, Color::RGBAu8(255, 0, 0, 255));
//                 }
//                 document.tile_manager.add_tile(&mut tile_grid, tile, Pos2::new(i, j));
//             }
//         }
// 
//         Self {
//             size,
//             tile_grid,
//         }
//     }
// }
// 
// impl Painter for ImagePainter {
// 
//     fn get_tilegrid(&self) -> &TileGrid {
//         &self.tile_grid
//     }
// 
// }