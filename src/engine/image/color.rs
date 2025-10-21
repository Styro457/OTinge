#[derive(Clone, Copy, Debug)]
pub enum Color {
    RGBAu8(u8, u8, u8, u8),
    RGBAf32(f32, f32, f32, f32),
    Ru8(u8),
}

impl Color {
    pub fn to_rgba_u8(&self) -> [u8; 4] {
        match self {
            Color::RGBAu8(r, g, b, a) => [*r, *g, *b, *a],
            Color::RGBAf32(r, g, b, a) => [
                (r * 255.0).round() as u8,
                (g * 255.0).round() as u8,
                (b * 255.0).round() as u8,
                (a * 255.0).round() as u8,
            ],
            Color::Ru8(r) => [*r, *r, *r, 255],
        }
    }
    
    pub fn to_rgba_f32(&self) -> [f32; 4] {
        match self {
            Color::RGBAu8(r, g, b, a) => [
                *r as f32 / 255.0,
                *g as f32 / 255.0,
                *b as f32 / 255.0,
                *a as f32 / 255.0,
            ],
            Color::RGBAf32(r, g, b, a) => [*r, *g, *b, *a],
            Color::Ru8(r) => {
                let rf = *r as f32 / 255.0;
                [rf, rf, rf, 1.0]
            }
        }
    }
    
    pub fn to_r_u8(&self) -> u8 {
        match self {
            Color::RGBAu8(r, _, _, _) => *r,
            Color::RGBAf32(r, _, _, _) => (*r * 255.0).round() as u8,
            Color::Ru8(r) => *r,
        }
    }
}
