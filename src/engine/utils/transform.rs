//TODO: Optimize

use super::math::pos2::Pos2;
use super::math::vec2::Vec2;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Transform {
    pub position: Pos2,
    pub scale: Vec2,
    pub rotation: f32, // in radians
}

impl Transform {
    pub fn new(x: u32, y: u32, rotation: f32, scale_x: f32, scale_y: f32) -> Transform {
        Transform {
            position: Pos2::new(x, y),
            scale: Vec2::new(scale_x, scale_y),
            rotation,
        }
    }

    pub fn zero() -> Transform {
        Transform {
            position: Pos2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
        }
    }
}