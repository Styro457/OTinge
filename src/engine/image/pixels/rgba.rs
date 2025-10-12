use super::super::pixel::Pixel;

pub trait Channel: Copy + Clone + Default {
    fn to_f32(self) -> f32;
    fn from_f32(f: f32) -> Self;
    fn max_value() -> f32;
}

#[derive(Copy, Clone, Default, Debug)]
pub struct RGBAPixel<T: Channel> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: Channel> RGBAPixel<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: Channel> Pixel for RGBAPixel<T> {
    type Channel = T;

    fn channel_count() -> usize { 4 }

    fn get_channel(&self, index: usize) -> T {
        match index {
            0 => self.r,
            1 => self.g,
            2 => self.b,
            3 => self.a,
            _ => panic!("Invalid channel index"),
        }
    }

    fn set_channel(&mut self, index: usize, value: T) {
        match index {
            0 => self.r = value,
            1 => self.g = value,
            2 => self.b = value,
            3 => self.a = value,
            _ => panic!("Invalid channel index"),
        }
    }

    fn blend(&self, background: &Self) -> Self {
        let alpha = self.a.to_f32() / T::max_value();
        let inv_alpha = 1.0 - alpha;

        let r = self.r.to_f32() * alpha + background.r.to_f32() * inv_alpha;
        let g = self.g.to_f32() * alpha + background.g.to_f32() * inv_alpha;
        let b = self.b.to_f32() * alpha + background.b.to_f32() * inv_alpha;
        let a = self.a.to_f32() + background.a.to_f32() * inv_alpha;

        Self {
            r: T::from_f32(r),
            g: T::from_f32(g),
            b: T::from_f32(b),
            a: T::from_f32(a),
        }
    }
}

impl Channel for u8 {
    fn to_f32(self) -> f32 { self as f32 }
    fn from_f32(f: f32) -> Self { f.round().clamp(0.0, 255.0) as u8 }
    fn max_value() -> f32 { 255.0 }
}

impl Channel for u16 {
    fn to_f32(self) -> f32 { self as f32 }
    fn from_f32(f: f32) -> Self { f.round().clamp(0.0, 65535.0) as u16 }
    fn max_value() -> f32 { 65535.0 }
}

impl Channel for f32 {
    fn to_f32(self) -> f32 { self }
    fn from_f32(f: f32) -> Self { f }
    fn max_value() -> f32 { 1.0 }
}
