use std::f64::INFINITY;

use macroquad::math::{DVec2, DVec4};

#[derive(Clone, Copy)]
pub struct Cell {
    pub vel: DVec2,
    vel_div: f64,
    vel_pot: f64,
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

const EPS: f64 = 1e-3;
const DIFF_K: f64 = 0.01;

pub fn simulate_frame(grid_1: &mut Vec<Vec<Cell>>, grid_2: &mut Vec<Vec<Cell>>, dt: f64) {
    simulate_diffusion(grid_1, grid_2);

    simulate_advection(grid_2, grid_1, dt);

    clear_velocity_divergence(grid_1, grid_2);

    // result is in ?
}

fn simulate_diffusion(grid_in: &Vec<Vec<Cell>>, grid_out: &mut Vec<Vec<Cell>>) {
    let mut delta_max: f64 = INFINITY;
    while delta_max > EPS {
        delta_max = 0.0;
        // don't simulate outer cells for predictable access to 4 neighbors
        for (y, inner_rows) in grid_in[1..grid_in.len() - 1].iter().enumerate() {
            for (x, _) in inner_rows[1..inner_rows.len() - 1].iter().enumerate() {
                // account for skipped 1st row/column
                delta_max = delta_max.max(diffuse_cell(x + 1, y + 1, grid_in, grid_out));
            }
        }
    }
}

fn simulate_advection(grid_in: &Vec<Vec<Cell>>, grid_out: &mut Vec<Vec<Cell>>, dt: f64) {
    for (y, inner_rows) in grid_in[1..grid_in.len() - 1].iter().enumerate() {
        for (x, _) in inner_rows[1..inner_rows.len() - 1].iter().enumerate() {
            // account for skipped 1st row/column
            advect_cell(x + 1, y + 1, grid_in, grid_out, dt);
        }
    }
}

fn clear_velocity_divergence(grid_1: &mut Vec<Vec<Cell>>, grid_2: &mut Vec<Vec<Cell>>) {
    for y in 1..grid_1.len() - 2 {
        for x in 1..grid_1[0].len() - 2 {
            // calculate velocity divergence of cell
            // account for skipped 1st row/column
            grid_1[y + 1][x + 1].vel_div = (grid_1[y + 1][x + 2].vel.x - grid_1[y + 1][x].vel.x
                + grid_1[y + 2][x + 1].vel.y
                - grid_1[y][x + 1].vel.y)
                * 0.5; // divide by 2 - x and y distance between cell's neighbors
            grid_2[y + 1][x + 1] = grid_1[y + 1][x + 1];
        }
    }

    let mut delta_max: f64 = INFINITY;
    let mut switch_grids = false;
    while delta_max > EPS {
        switch_grids = !switch_grids;
        delta_max = 0.0;

        for y in 1..grid_1.len() - 2 {
            for x in 1..grid_1[0].len() - 2 {
                // account for skipped 1st row/column
                delta_max = match switch_grids {
                    true => delta_max.max(update_cell_vel_pot(x + 1, y + 1, grid_2, grid_1)),
                    false => delta_max.max(update_cell_vel_pot(x + 1, y + 1, grid_1, grid_2)),
                };
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
    let curr = &grid_in[y][x];
    let n_left = &grid_in[y][x - 1];
    let n_right = &grid_in[y][x + 1];
    let n_top = &grid_in[y - 1][x];
    let n_bottom = &grid_in[y + 1][x];
    let old_speed = curr.vel.length();

    grid_out[y][x].vel = (curr.vel
        + DIFF_K * (n_left.vel + n_right.vel + n_top.vel + n_bottom.vel) * 0.25)
        / (DIFF_K + 1.0);
    grid_out[y][x].color = (curr.color
        + DIFF_K * (n_left.color + n_right.color + n_top.color + n_bottom.color) * 0.25)
        / (DIFF_K + 1.0);

    return curr.vel.length() - old_speed;
}

fn advect_cell(
    x: usize,
    y: usize,
    grid_in: &Vec<Vec<Cell>>,
    grid_out: &mut Vec<Vec<Cell>>,
    dt: f64,
) {
    // (0;0) is the middle of the left-top (never processed) cell

    // calculate source point for current cell
    let source = DVec2 {
        x: x as f64,
        y: y as f64,
    } - grid_in[y][x].vel * dt;

    // as cell size is 1:1 and integer coordinates point at cell centers,
    // 4 closest cells to source are cell with (source.x;source.y) integer parts and 3 more cells:
    // to the right, bottom and bottom-right of it

    // get top-left cell indexes
    let tl_x = (source.x as usize).clamp(0, grid_in[0].len() - 2);
    let tl_y = (source.y as usize).clamp(0, grid_in.len() - 2);

    // get interpolation coefficients
    let int_x = source.x.fract();
    let int_y = source.y.fract();

    // lerp coefficients for each neighbor
    let k_top_left = (1.0 - int_x) * (1.0 - int_y);
    let k_top_right = int_x * (1.0 - int_y);
    let k_bottom_left = (1.0 - int_x) * int_y;
    let k_bottom_right = int_x * int_y;

    grid_out[y][x].vel = grid_in[tl_y][tl_x].vel * k_top_left
        + grid_in[tl_y][tl_x + 1].vel * k_top_right
        + grid_in[tl_y + 1][tl_x].vel * k_bottom_left
        + grid_in[tl_y + 1][tl_x + 1].vel * k_bottom_right;
    grid_out[y][x].color = grid_in[tl_y][tl_x].color * k_top_left
        + grid_in[tl_y][tl_x + 1].color * k_top_right
        + grid_in[tl_y + 1][tl_x].color * k_bottom_left
        + grid_in[tl_y + 1][tl_x + 1].color * k_bottom_right;
}

// returns velocity potential delta
fn update_cell_vel_pot(
    x: usize,
    y: usize,
    grid_in: &Vec<Vec<Cell>>,
    grid_out: &mut Vec<Vec<Cell>>,
) -> f64 {
    let old_pot = grid_in[y][x].vel_pot;
    grid_out[y][x].vel_pot = (grid_in[y][x - 1].vel_pot
        + grid_in[y][x + 1].vel_pot
        + grid_in[y - 1][x].vel_pot
        + grid_in[y + 1][x].vel_pot
        - grid_in[y][x].vel_div)
        * 0.25;
    return grid_out[y][x].vel_pot - old_pot;
}
