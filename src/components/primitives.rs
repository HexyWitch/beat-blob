#[derive(PartialEq)]
pub enum FillMode {
    Filled,
    Outline(f32),
}

pub struct ColoredRect {
    pub rect: (f32, f32, f32, f32),
    pub color: (f32, f32, f32, f32),
}
pub struct ColoredCircle {
    pub radius: f32,
    pub color: (f32, f32, f32, f32),
    pub fill: FillMode,
}
