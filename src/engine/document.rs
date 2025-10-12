use super::image::pixel::Pixel;
use super::layer::Layer;
use super::image::buffer::ImageBuffer;

pub struct Document<P: Pixel> {
    pub width: usize,
    pub height: usize,
    pub base_layer: Layer<P>,
}

impl<P: Pixel> Document<P> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            base_layer: Layer::new("Base Layer", width, height, None),
        }
    }

    pub fn render(&self) -> ImageBuffer<P> {
        self.base_layer.render()
    }
}