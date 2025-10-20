use crate::engine::composition::BlendMode;
use crate::engine::layer::effects::Effect;
use crate::engine::utils::transform::Transform;

use super::painter::Painter;

pub struct Layer {
    pub label: String,
    pub show: bool,
    pub transform: Transform,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub painter: Option<Box<dyn Painter>>,
    pub effects: Vec<Box<dyn Effect>>,
    pub children: Vec<Layer>
}