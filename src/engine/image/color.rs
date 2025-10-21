#[derive(Clone, Copy, Debug)]
pub enum Color {
    RGBAu8([u8; 4]),
    RGBAf32([f32; 4]),
    Ru8(u8),
}

impl Color {
    pub fn to_rgba_u8(&self) -> [u8; 4] {
        match self {
            Color::RGBAu8(rgba) => *rgba,
            Color::RGBAf32(rgba) => [
                (rgba[0] * 255.0).round() as u8,
                (rgba[1] * 255.0).round() as u8,
                (rgba[2] * 255.0).round() as u8,
                (rgba[3] * 255.0).round() as u8,
            ],
            Color::Ru8(r) => [*r, *r, *r, 255],
        }
    }
    
    pub fn to_rgba_f32(&self) -> [f32; 4] {
        match self {
            Color::RGBAu8(rgba) => [
                rgba[0] as f32 / 255.0,
                rgba[1] as f32 / 255.0,
                rgba[2] as f32 / 255.0,
                rgba[3] as f32 / 255.0,
            ],
            Color::RGBAf32(rgba) => *rgba,
            Color::Ru8(r) => {
                let rf = *r as f32 / 255.0;
                [rf, rf, rf, 1.0]
            }
        }
    }
    
    pub fn to_r_u8(&self) -> u8 {
        match self {
            Color::RGBAu8(rgba) => rgba[0],
            Color::RGBAf32(rgba) => (rgba[0] * 255.0).round() as u8,
            Color::Ru8(r) => *r,
        }
    }
}
