//TODO: Optimize with pooling
pub struct Transform {
    pub x: usize,
    pub y: usize,
    pub rotation: f32, // in radians
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Transform {
    pub fn new(x: usize, y: usize, rotation: f32, scale_x: f32, scale_y: f32) -> Transform {
        Transform {
            x,
            y,
            rotation,
            scale_x,
            scale_y,
        }
    }

    pub fn zero() -> Transform {
        Transform {
            x: 0,
            y: 0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }
}