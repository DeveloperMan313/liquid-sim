use std::f64::INFINITY;

use macroquad::math::{DVec2, DVec4};

#[derive(Clone)]
pub struct Cell {
    pub vel: DVec2,
    pub color: DVec4,
}

impl Cell {
    pub fn new() -> Cell {
        return Cell {
            vel: DVec2::ZERO,
            color: DVec4::ZERO,
        };
    }
}

const EPS: f64 = 1e-3;
const DIFF_K: f64 = 0.01;

pub fn simulate_frame(grid_in: &Vec<Vec<Cell>>, grid_out: &mut Vec<Vec<Cell>>, _dt: f64) {
    simulate_diffusion(grid_in, grid_out);

    // simulate advection

    // clear vel divergence
}

fn simulate_diffusion(grid_in: &Vec<Vec<Cell>>, grid_out: &mut Vec<Vec<Cell>>) {
    let mut delta_max: f64 = INFINITY;
    while delta_max > EPS {
        delta_max = 0.0;
        // don't simulate outer cells for predictable access to 4 neighbors
        for (y, inner_rows) in grid_in[1..grid_in.len() - 1].iter().enumerate() {
            for (x, _) in inner_rows[1..inner_rows.len() - 1].iter().enumerate() {
                // account for skipped 1st element
                delta_max = delta_max.max(diffuse_cell(x + 1, y + 1, grid_in, grid_out));
            }
        }
    }
}

// returns speed delta
fn diffuse_cell(
    x: usize,
    y: usize,
    grid_in: &Vec<Vec<Cell>>,
    grid_out: &mut Vec<Vec<Cell>>,
) -> f64 {
    let curr = &mut grid_out[y][x];
    let n_left = &grid_in[y][x - 1];
    let n_right = &grid_in[y][x + 1];
    let n_top = &grid_in[y - 1][x];
    let n_bottom = &grid_in[y + 1][x];
    let old_speed = curr.vel.length();

    curr.vel = (curr.vel + DIFF_K * (n_left.vel + n_right.vel + n_top.vel + n_bottom.vel) * 0.25)
        / (DIFF_K + 1.0);
    curr.color = (curr.color
        + DIFF_K * (n_left.color + n_right.color + n_top.color + n_bottom.color) * 0.25)
        / (DIFF_K + 1.0);

    return curr.vel.length() - old_speed;
}
