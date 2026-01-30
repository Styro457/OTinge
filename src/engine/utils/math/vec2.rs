#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

//TODO: Implement operators and other methods for Vec2
impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

}

impl From<(f32, f32)> for Vec2 {
    fn from(tuple: (f32, f32)) -> Self {
        Vec2 { x: tuple.0, y: tuple.1 }
    }
}

impl From<Vec2> for [f32; 2] {
    fn from(pos: Vec2) -> Self {
        [pos.x, pos.y]
    }
}