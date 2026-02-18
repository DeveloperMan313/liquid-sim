use macroquad::prelude::*;
use std::thread;

use crate::{
    entities::{BrushState, Cell, RenderView, SimConfig},
    simulation::simulate_frame,
};

pub async fn run() {
    let mut conf = SimConfig::new();
    let mut brush = BrushState::new();

    let mut grid_input = vec![vec![Cell::new(); conf.grid_width]; conf.grid_height]; // user input deltas, reset every iteration
    let mut grid_sim = grid_input.clone(); // simulation buffer, in/out
    let mut grid_sim_inner = grid_input.clone(); // inner simulation buffer
    let mut grid_render = grid_input.clone(); // rendered on screen

    // TODO remove this
    request_new_screen_size(
        (conf.grid_width * conf.cell_size_px) as f32,
        (conf.grid_height * conf.cell_size_px) as f32,
    );

    // for demo
    for y in 1..grid_sim.len() - 1 {
        for x in 1..grid_sim[0].len() - 1 {
            if x > conf.grid_width / 2 {
                grid_sim[y][x].color[0] = 1.0;
                grid_sim[y][x].color[3] = 1.0;
            }
            if y > conf.grid_height / 2 {
                grid_sim[y][x].color[1] = 1.0;
                grid_sim[y][x].color[3] = 1.0;
            }
        }
    }

    let mut sim_dt: f64 = 0.0;

    loop {
        for y in 0..grid_sim.len() {
            for x in 0..grid_sim[0].len() {
                grid_sim[y][x].vel += grid_input[y][x].vel;
            }
        }

        let sim_handle = thread::spawn(move || {
            simulate_frame(&mut grid_sim, &mut grid_sim_inner, sim_dt, conf);
            (grid_sim, grid_sim_inner)
        });

        sim_dt = 0.0;

        while !sim_handle.is_finished() {
            let dt = get_frame_time() as f64;
            sim_dt += dt;

            handle_input(&mut grid_input, &mut grid_render, dt, &mut brush, &mut conf);

            render_grid(&grid_render, &conf);

            next_frame().await;
        }

        (grid_sim, grid_sim_inner) = sim_handle
            .join()
            .expect("Couldn't join on the simulation thread");

        for y in 0..grid_render.len() {
            for x in 0..grid_render[0].len() {
                grid_render[y][x] = grid_sim[y][x];
            }
        }
    }
}

fn handle_input(
    grid_input: &mut Vec<Vec<Cell>>,
    grid_render: &mut Vec<Vec<Cell>>,
    dt: f64,
    brush: &mut BrushState,
    conf: &mut SimConfig,
) {
    // clear grid_input velocity
    for row in grid_input.iter_mut() {
        for cell in row.iter_mut() {
            cell.vel = DVec2::ZERO;
        }
    }

    // clear last square brush
    let (mut cx, mut cy) = brush.prev_center;
    for y in (cy - brush.size).max(1)..=(cy + brush.size).min(conf.grid_height as i32 - 2) {
        for x in (cx - brush.size).max(1)..=(cx + brush.size).min(conf.grid_width as i32 - 2) {
            if x == cx - brush.size
                || x == cx + brush.size
                || y == cy - brush.size
                || y == cy + brush.size
            {
                grid_render[y as usize][x as usize].brush_outline = false;
            }
        }
    }

    match mouse_wheel() {
        (_, y) if y > 0.0 && brush.size < 30 => {
            brush.size += 1;
        }
        (_, y) if y < 0.0 && brush.size > 0 => {
            brush.size -= 1;
        }
        (_, _) => (),
    }

    let brush_was_drawing = brush.is_drawing;
    brush.is_drawing = is_mouse_button_down(MouseButton::Left);

    // draw current square brush
    (cx, cy) = window_coords_to_grid_coords(mouse_position(), conf);
    brush.prev_center = (cx, cy);
    for y in (cy - brush.size).max(1)..=(cy + brush.size).min(conf.grid_height as i32 - 2) {
        for x in (cx - brush.size).max(1)..=(cx + brush.size).min(conf.grid_width as i32 - 2) {
            if x == cx - brush.size
                || x == cx + brush.size
                || y == cy - brush.size
                || y == cy + brush.size
            {
                grid_render[y as usize][x as usize].brush_outline = true;
            }
        }
    }

    // velocity mode
    if brush_was_drawing && brush.is_drawing {
        let mouse_delta = DVec2::from(mouse_delta_position());
        let vel_delta = -mouse_delta / dt * conf.brush_vel_mult;
        // square brush
        for y in (cy - brush.size).max(1)..=(cy + brush.size).min(conf.grid_height as i32 - 2) {
            for x in (cx - brush.size).max(1)..=(cx + brush.size).min(conf.grid_width as i32 - 2) {
                grid_input[y as usize][x as usize].vel += vel_delta;
            }
        }
    }

    if let Some(ch) = get_char_pressed() {
        match ch {
            '1' => conf.render_view = RenderView::Color,
            '2' => conf.render_view = RenderView::Speed,
            '3' => conf.render_view = RenderView::Velocity,
            _ => (),
        }
    }
}

fn render_grid(grid: &Vec<Vec<Cell>>, conf: &SimConfig) {
    let texture_bytes: Vec<u8> = grid
        .iter()
        .flatten()
        .map(|cell: &Cell| match conf.render_view {
            RenderView::Color => (cell.color.to_array(), cell.brush_outline),
            RenderView::Speed => {
                let speed = cell.vel.length();
                let speed_0_to_1 = speed / (speed + 1.0);
                ((DVec4::ONE * speed_0_to_1).to_array(), cell.brush_outline)
            }
            RenderView::Velocity => {
                let vel_minus_1_to_1 = cell.vel / (cell.vel.length() + 1.0);
                (
                    dvec4(
                        vel_minus_1_to_1.x.abs(),
                        vel_minus_1_to_1.y.abs(),
                        vel_minus_1_to_1.dot(dvec2(1.0, 1.0)),
                        1.0,
                    )
                    .to_array(),
                    cell.brush_outline,
                )
            }
        })
        .map(|(color, brush_outline)| match brush_outline {
            true => [1.0 - color[0], 1.0 - color[1], 1.0 - color[2], 1.0],
            false => color,
        })
        .flatten()
        .map(|f: f64| (f * 256.) as u8)
        .collect();

    let texture = Texture2D::from_rgba8(
        conf.grid_width as u16,
        conf.grid_height as u16,
        &texture_bytes,
    );
    texture.set_filter(FilterMode::Nearest);

    let texture_size = Vec2 {
        x: (conf.grid_width * conf.cell_size_px) as f32,
        y: (conf.grid_height * conf.cell_size_px) as f32,
    };

    draw_texture_ex(
        &texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(texture_size),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}

fn window_coords_to_grid_coords(coords: (f32, f32), conf: &SimConfig) -> (i32, i32) {
    let cell_size = conf.cell_size_px as i32;
    (coords.0 as i32 / cell_size, coords.1 as i32 / cell_size)
}
