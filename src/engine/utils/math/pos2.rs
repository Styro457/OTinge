#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Pos2 {
    pub x: u32,
    pub y: u32,
}

//TODO: Implement operators and other methods for Vec2
impl Pos2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: u32, y: u32) -> Pos2 {
        Pos2 { x, y }
    }
}

impl From<(u32, u32)> for Pos2 {
    fn from(tuple: (u32, u32)) -> Self {
        Pos2 { x: tuple.0, y: tuple.1 }
    }
}

