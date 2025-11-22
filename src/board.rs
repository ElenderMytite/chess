use bevy::prelude::*;
const LIGHT: Color = Color::srgb(0.97, 0.87, 0.68);
const DARK: Color = Color::srgb(0.64, 0.44, 0.27);
pub fn setup_board_and_camera(mut commands: Commands) {
    // Spawn orthographic camera
    commands.spawn( Camera2d::default() );
    // Spawn the chessboard grid (8x8)
    let board_size = 8;
    let square_size = 100.0;
    let half = square_size * board_size as f32 / 2.0;

    // Spawn 64 squares (8x8 grid)
    for i in 0..8 {
        for j in 0..8 {
            let color = if (i + j) % 2 == 0 {
                LIGHT
            } else {
                DARK
            };

            commands.spawn( (Sprite{
                color: color,
                custom_size: Some(Vec2::splat(square_size)),
                ..Default::default()
                },
                Transform::from_xyz(
                    i as f32 * square_size - half + square_size / 2.0,
                    j as f32 * square_size - half + square_size / 2.0,
                    0.0,)
            ));
        }
    }
}