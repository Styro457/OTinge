#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Lighten,
    Darken,
    ColorDodge,
    ColorBurn,
    Subtract,
    Add,
    Difference,
}