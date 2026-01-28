use crate::engine::composition::BlendMode;
use crate::engine::utils::transform::Transform;

use crate::engine::effects::Effect;
use crate::engine::painters::Painter;

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

impl Layer {
    pub fn new(label: &str, painter: Option<Box<dyn Painter>>) -> Self {
        Self {
            label: label.to_string(),
            show: true,
            transform: Transform::zero(),
            opacity: 1.0,
            blend_mode: BlendMode::Overlay,
            painter,
            effects: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn get_layer_count(&self) -> u32 {
        let mut count = 1;
        for child in &self.children {
            count += child.get_layer_count();
        }
        count
    }

    pub fn add_child(&mut self, layer: Layer) {
        self.children.push(layer);
    }
}