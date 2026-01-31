use macroquad::{prelude::*, rand::rand};

use crate::simulation::{Cell, simulate_frame};

pub async fn run() {
    // init flow grid
    let grid_width = 128;
    let grid_height = 128;
    let cell_size_px = 4;
    let mut grid_1 = vec![vec![Cell::new(); grid_width]; grid_height];
    let mut grid_2 = vec![vec![Cell::new(); grid_width]; grid_height];
    let dt = 0.1;

    let texture_size = Vec2 {
        x: (grid_width * cell_size_px) as f32,
        y: (grid_height * cell_size_px) as f32,
    };

    // TODO move into window config
    request_new_screen_size(texture_size.x, texture_size.y);

    // for demo
    for row in grid_1[1..grid_height - 1].iter_mut() {
        for cell in row[1..grid_width - 1].iter_mut() {
            cell.vel = DVec2 {
                x: 1.0,
                y: 0.2 * (rand() as f64) / u32::MAX as f64 - 0.1,
            };
            cell.color = DVec4 {
                x: (rand() as f64) / u32::MAX as f64,
                y: (rand() as f64) / u32::MAX as f64,
                z: (rand() as f64) / u32::MAX as f64,
                w: 1.0,
            };
        }
    }

    loop {
        // simulate frame
        simulate_frame(&mut grid_1, &mut grid_2, dt);

        // render frame
        let texture_bytes: Vec<u8> = grid_1
            .iter()
            .flatten()
            .map(|cell: &Cell| cell.color.to_array())
            .flatten()
            .map(|f: f64| (f * 256.) as u8)
            .collect();

        let texture = Texture2D::from_rgba8(grid_width as u16, grid_height as u16, &texture_bytes);
        texture.set_filter(FilterMode::Nearest);

        clear_background(BLACK);

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
