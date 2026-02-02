use macroquad::prelude::*;

use crate::{
    entities::{Cell, RenderView, SimConfig},
    simulation::simulate_frame,
};

pub async fn run() {
    let conf = SimConfig::new();

    let mut grid_1 = vec![vec![Cell::new(); conf.grid_width]; conf.grid_height];
    let mut grid_2 = grid_1.clone();

    let dt = 0.1;

    let texture_size = Vec2 {
        x: (conf.grid_width * conf.cell_size_px) as f32,
        y: (conf.grid_height * conf.cell_size_px) as f32,
    };

    // TODO remove this
    request_new_screen_size(texture_size.x, texture_size.y);

    // for demo
    for y in 1..grid_1.len() - 1 {
        for x in 1..grid_1[0].len() - 1 {
            if x > 4 && x < 64 - 4 && y > 32 - 4 && y < 32 + 4 {
                if x < 32 - 4 {
                    grid_1[y][x].vel = DVec2 { x: 5.0, y: 1.0 };
                    grid_1[y][x].color = dvec4(1.0, 0.0, 0.0, 1.0);
                }
                if x > 32 + 4 {
                    grid_1[y][x].vel = DVec2 { x: -0.5, y: -1.0 };
                    grid_1[y][x].color = dvec4(0.0, 1.0, 0.0, 1.0);
                }
            }
        }
    }

    loop {
        // simulate frame
        simulate_frame(&mut grid_1, &mut grid_2, dt, &conf);

        // render frame
        let texture_bytes: Vec<u8> = grid_1
            .iter()
            .flatten()
            .map(|cell: &Cell| match conf.render_view {
                RenderView::Color => cell.color.to_array(),
                RenderView::Speed => {
                    let speed = cell.vel.length();
                    let speed_0_to_1 = speed / (speed + 1.0);
                    (DVec4::ONE * speed_0_to_1).to_array()
                }
                RenderView::Velocity => {
                    let vel_minus_1_to_1 = cell.vel / (cell.vel.length() + 1.0);
                    dvec4(
                        vel_minus_1_to_1.x.abs(),
                        vel_minus_1_to_1.y.abs(),
                        vel_minus_1_to_1.dot(dvec2(1.0, 1.0)),
                        1.0,
                    )
                    .to_array()
                }
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

        next_frame().await
    }
}
