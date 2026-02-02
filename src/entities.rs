use macroquad::math::{DVec2, DVec4};

#[derive(Clone, Copy)]
pub struct Cell {
    pub vel: DVec2,
    pub vel_div: f64,
    pub vel_pot: f64,
    pub color: DVec4,
}

impl Cell {
    pub fn new() -> Cell {
        return Cell {
            vel: DVec2::ZERO,
            vel_div: 0.0,
            vel_pot: 0.0,
            color: DVec4::ZERO,
        };
    }
}

#[allow(dead_code)]
pub enum RenderView {
    Color,
    Speed,
    Velocity,
}
