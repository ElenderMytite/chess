use bevy::prelude::*;
use crate::atlas::PieceAtlas;
pub fn setup_pieces(mut commands: Commands, atlas: ResMut<PieceAtlas>) {
    const WHITE: usize = 0;
    const BLACK: usize = 6;
    const SQUARE_SIZE: f32 = 100.0;
    let half = SQUARE_SIZE * 4.0;
    let mut spawn_piece = |x: i32, y: i32, index: usize| {
        let px = x as f32 * SQUARE_SIZE - half + SQUARE_SIZE / 2.0;
        let py = y as f32 * SQUARE_SIZE - half + SQUARE_SIZE / 2.0;

        commands.spawn((
            Sprite::from_atlas_image(atlas.texture.clone(), TextureAtlas{layout: atlas.layout.clone(), index}),
            Transform::from_xyz(px, py, 1.0).with_scale(Vec3::splat(0.5),
        )));
    };

    // Pawns
    for file in 0..8 {
        spawn_piece(file, 1, WHITE + 5); // white pawn
        spawn_piece(file, 6, BLACK + 5); // black pawn
    }

    // Rooks
    spawn_piece(0, 0, WHITE + 4);
    spawn_piece(7, 0, WHITE + 4);
    spawn_piece(0, 7, BLACK + 4);
    spawn_piece(7, 7, BLACK + 4);

    // Knights
    spawn_piece(1, 0, WHITE + 3);
    spawn_piece(6, 0, WHITE + 3);
    spawn_piece(1, 7, BLACK + 3);
    spawn_piece(6, 7, BLACK + 3);

    // Bishops
    spawn_piece(2, 0, WHITE + 2);
    spawn_piece(5, 0, WHITE + 2);
    spawn_piece(2, 7, BLACK + 2);
    spawn_piece(5, 7, BLACK + 2);

    // Queens
    spawn_piece(3, 0, WHITE + 1);
    spawn_piece(3, 7, BLACK + 1);

    // Kings
    spawn_piece(4, 0, WHITE + 0);
    spawn_piece(4, 7, BLACK + 0);
}