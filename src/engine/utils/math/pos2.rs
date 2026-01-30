use crate::engine::utils::math::vec2::Vec2;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Pos2 {
    pub x: u32,
    pub y: u32,
}

//TODO: Implement operators and other methods for Pos2
impl Pos2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: u32, y: u32) -> Pos2 {
        Pos2 { x, y }
    }

    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl From<(u32, u32)> for Pos2 {
    fn from(tuple: (u32, u32)) -> Self {
        Pos2 { x: tuple.0, y: tuple.1 }
    }
}

impl From<Pos2> for [u32; 2] {
    fn from(pos: Pos2) -> Self {
        [pos.x, pos.y]
    }
}

impl From<Pos2> for [f32; 2] {
    fn from(pos: Pos2) -> Self {
        [pos.x as f32, pos.y as f32]
    }
}

// Implement operators
impl std::ops::Div for Pos2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl std::ops::Mul for Pos2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl std::ops::Add for Pos2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pos2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add<u32> for Pos2 {
    type Output = Self;
    fn add(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}



