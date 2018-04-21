#[derive(Clone, Copy, PartialEq)]
pub enum PadTeam {
    Blue,
    Red,
    Green,
    Yellow,
}

impl PadTeam {
    pub fn color(&self) -> (f32, f32, f32, f32) {
        match *self {
            PadTeam::Blue => (0.5, 0.5, 1.0, 1.0),
            PadTeam::Red => (1.0, 0.5, 0.5, 1.0),
            PadTeam::Green => (0.5, 1.0, 0.5, 1.0),
            PadTeam::Yellow => (1.0, 1.0, 0.5, 1.0),
        }
    }
}

pub struct Pad {
    pub triggered: bool,
    pub pulse_timer: f32,
}
