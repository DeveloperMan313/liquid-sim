use macroquad::math::{DVec2, DVec4};

#[derive(Clone)]
pub struct Cell {
    pub vel: DVec2,
    pub vel_div: f64,
    pub vel_pot: f64,
    pub color: DVec4,
    pub brush_outline: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            vel: DVec2::ZERO,
            vel_div: 0.0,
            vel_pot: 0.0,
            color: DVec4::ZERO,
            brush_outline: false,
        }
    }
}

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
    pub speed: f64,
    pub render_view: RenderView,
    pub brush_vel_mult: f64,
}

impl SimConfig {
    pub fn new() -> SimConfig {
        SimConfig {
            grid_width: 64,
            grid_height: 64,
            cell_size_px: 8,
            eps: 1e-2,
            diff_k: 1e-2,
            speed: 5.0,
            render_view: RenderView::Color,
            brush_vel_mult: 0.1,
        }
    }
}

pub struct BrushState {
    pub size: i32,
    pub is_drawing: bool,
    pub prev_center: (i32, i32),
}

impl BrushState {
    pub fn new() -> BrushState {
        BrushState {
            size: 3,
            is_drawing: false,
            prev_center: (0, 0),
        }
    }
}
