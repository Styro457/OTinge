use std::ops::{Index, IndexMut};
use super::pixel::Pixel;

pub struct ImageBuffer<P: Pixel> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<P>,
}

impl<P: Pixel> ImageBuffer<P> {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            data: vec![P::default(); size],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &P {
        let idx = y * self.width + x;
        &self.data[idx]
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut P {
        let idx = y * self.width + x;
        &mut self.data[idx]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: P) {
        let idx = y * self.width + x;
        self.data[idx] = pixel;
    }

    pub fn clear(&mut self, color: P) {
        for px in &mut self.data {
            *px = color;
        }
    }

}

impl<P: Pixel> Index<(usize, usize)> for ImageBuffer<P> {
    type Output = P;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get_pixel(x, y)
    }
}

impl<P: Pixel> IndexMut<(usize, usize)> for ImageBuffer<P> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.get_pixel_mut(x, y)
    }
}
