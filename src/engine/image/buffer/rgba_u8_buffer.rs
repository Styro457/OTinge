use image::DynamicImage;
use wgpu::TextureFormat;
use crate::engine::document::Document;
use crate::engine::image::buffer::{ImageBuffer};
use crate::engine::image::color::Color;
use crate::engine::image::tiles::grid::TileGrid;
use crate::engine::utils::math::pos2::Pos2;

pub struct RGBAu8Buffer {
    data: Vec<u8>,
}

impl RGBAu8Buffer {
    fn create_from_data(data: Vec<u8>) -> Box<dyn ImageBuffer> {
        Box::new(Self {
            data,
        })
    }
}

impl ImageBuffer for RGBAu8Buffer {

    fn new(size: Pos2) -> Self {
        Self {
            data: vec![0; (size.x * size.y * 4) as usize],
        }
    }

    fn create(&self, size: Pos2) -> Box<dyn ImageBuffer> {
        Box::new(Self::new(size))
    }

    fn get_wgpu_format(&self) -> TextureFormat {
        TextureFormat::Rgba8UnormSrgb
    }

    fn get_bytes_per_pixel(&self) -> u32 {
        4
    }

    fn get_data(&self) -> &[u8] {
        &*self.data
    }

    fn set_pixel(&mut self, index: usize, color: Color) {
        let offset = index * 4;
        self.data[offset..(offset + 4)].copy_from_slice(&color.to_rgba_u8());
    }

    fn len(&self) -> usize {
        self.data.len()/4
    }

    fn convert_dynamic_image(&self, document: &mut Document, image: &DynamicImage) -> (Pos2, TileGrid) {
        let converted_img = image.to_rgba8();

        let mut tile_grid = document.tile_manager.create_tile_grid(document.size);

        let tile_size = document.tile_manager.get_tile_size(); // e.g., 256

        // 2. Iterate through the grid coordinates
        for y in 0..tile_grid.size.y {
            for x in 0..tile_grid.size.x {
                // Calculate pixel bounds for this tile
                let start_x = x * tile_size.x;
                let start_y = y * tile_size.y;

                // Create the buffer for the tile
                // document.buffer_type.create creates a blank 256x256 buffer
                let tile = document.tile_manager.get_tile_or_create(&mut tile_grid, x, y);

                // 3. Copy pixels from the DynamicImage into the Tile buffer
                // We use a nested loop for the 256x256 area
                for py in 0..tile_size.y {
                    for px in 0..tile_size.x {
                        let gx = start_x + px;
                        let gy = start_y + py;

                        // Only copy if we are within the actual image bounds
                        if gx < converted_img.width() && gy < converted_img.height() {
                            let pixel = converted_img.get_pixel(gx, gy);
                            let color = Color::RGBAu8(pixel[0], pixel[1], pixel[2], pixel[3]);

                            let local_index = (py * tile_size.x + px) as usize;
                            tile.set_pixel(local_index, color);
                        }
                    }
                }
            }
        }

        (Pos2::new(image.width(), image.height()), tile_grid)
    }
}