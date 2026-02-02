use macroquad::math::{DVec2, DVec4};

#[derive(Clone)]
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

pub struct SimConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size_px: usize,
    pub eps: f64,
    pub diff_k: f64,
    pub render_view: RenderView,
}

impl SimConfig {
    pub fn new() -> SimConfig {
        return SimConfig {
            grid_width: 64,
            grid_height: 64,
            cell_size_px: 8,
            eps: 1e-2,
            diff_k: 1e-2,
            render_view: RenderView::Color,
        };
    }
}
