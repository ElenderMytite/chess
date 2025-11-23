use bevy::prelude::*;
use crate::atlas::PieceAtlas;
#[derive(Clone,Resource)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Clone,Resource)]
pub enum PieceColor {
    White,
    Black,
    
}
fn index_from_piece_type(piece_type: &PieceType, piece_color: &PieceColor) -> usize {
    let base_index = match piece_type {
        PieceType::King => 0,
        PieceType::Queen => 1,
        PieceType::Rook => 4,
        PieceType::Bishop => 2,
        PieceType::Knight => 3,
        PieceType::Pawn => 5,
    };
    match piece_color {
        PieceColor::White => base_index,
        PieceColor::Black => base_index + 6,
    }
}
#[derive(Component,Clone)]
pub struct PieceInfo(pub UVec2, pub PieceType, pub PieceColor);

pub fn setup_pieces(mut commands: Commands, atlas: ResMut<PieceAtlas>) {
    const SQUARE_SIZE: f32 = 100.0;
    let half = SQUARE_SIZE * 4.0;
    let mut spawn_piece = |x: u32, y: u32, piece_type: PieceType, piece_color: PieceColor| {
        let px = x as f32 * SQUARE_SIZE - half + SQUARE_SIZE / 2.0;
        let py = y as f32 * SQUARE_SIZE - half + SQUARE_SIZE / 2.0;

        commands.spawn((
            Sprite::from_atlas_image(atlas.texture.clone(), TextureAtlas{layout: atlas.layout.clone(), index: index_from_piece_type(&piece_type, &piece_color)}),
            Transform::from_xyz(px, py, 1.0).with_scale(Vec3::splat(0.5)),
        )).insert(PieceInfo(UVec2::new(x, y), piece_type.clone(), piece_color.clone()));
    };

    // Pawns
    for file in 0..8 {
        spawn_piece(file, 1, PieceType::Pawn, PieceColor::White); // white pawn
        spawn_piece(file, 6, PieceType::Pawn, PieceColor::Black); // black pawn
    }

    // Rooks
    spawn_piece(0, 0, PieceType::Rook, PieceColor::White);
    spawn_piece(7, 0, PieceType::Rook, PieceColor::White);
    spawn_piece(0, 7, PieceType::Rook, PieceColor::Black);
    spawn_piece(7, 7, PieceType::Rook, PieceColor::Black);

    // Knights
    spawn_piece(1, 0, PieceType::Knight, PieceColor::White);
    spawn_piece(6, 0, PieceType::Knight, PieceColor::White);
    spawn_piece(1, 7, PieceType::Knight, PieceColor::Black);
    spawn_piece(6, 7, PieceType::Knight, PieceColor::Black);

    // Bishops
    spawn_piece(2, 0, PieceType::Bishop, PieceColor::White);
    spawn_piece(5, 0, PieceType::Bishop, PieceColor::White);
    spawn_piece(2, 7, PieceType::Bishop, PieceColor::Black);
    spawn_piece(5, 7, PieceType::Bishop, PieceColor::Black);
    
    // Queens
    spawn_piece(3, 0, PieceType::Queen, PieceColor::White);
    spawn_piece(3, 7, PieceType::Queen, PieceColor::Black);

    // Kings
    spawn_piece(4, 0, PieceType::King, PieceColor::White);
    spawn_piece(4, 7, PieceType::King, PieceColor::Black);

    // Queens
    spawn_piece(3, 0, PieceType::Queen, PieceColor::White);
    spawn_piece(3, 7, PieceType::Queen, PieceColor::Black);

    // Kings
    spawn_piece(4, 0, PieceType::King, PieceColor::White);
    spawn_piece(4, 7, PieceType::King, PieceColor::Black);
}