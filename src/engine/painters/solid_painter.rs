use crate::engine::image::buffer::ImageBuffer;
use crate::engine::image::pixel::Pixel;
use super::super::layer::LayerPainter;

pub struct SolidPainter<P: Pixel> {
    color: P,
}

impl<P: Pixel> SolidPainter<P> {
    pub fn new(color: P) -> Self {
        Self { color }
    }
}

impl<P: Pixel> LayerPainter<P> for SolidPainter<P> {
    fn paint(&self, buffer: &mut ImageBuffer<P>) {
        buffer.clear(self.color);
    }
}