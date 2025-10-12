use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::pixel::Pixel;
use super::super::layer::LayerPainter;

pub struct ImagePainter {
    
}

impl<P: Pixel> LayerPainter<P> for ImagePainter {
    fn paint(&self, buffer: &mut ImageBuffer<P>) {
    }
}