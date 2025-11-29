use bevy::prelude::*;

use crate::pieces::{PieceColor, PieceInfo, PieceType};
#[derive(Resource, Clone)]
pub struct SelectedPiece(pub Option<PieceInfo>);
#[derive(Resource, Clone, PartialEq, Eq)]
pub struct Turn(pub PieceColor);

pub fn selection(
    mut turn: ResMut<Turn>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut selected_piece: ResMut<SelectedPiece>,
    commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    pieces: Query<(Entity, &mut PieceInfo, &mut Transform)>,
) {
    let get_mouse_pos = || -> Option<Vec2> {
        let window = windows.single().unwrap();
        if let Some(screen_pos) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single().unwrap();
            if let Ok(world_pos) = camera.viewport_to_world(camera_transform, screen_pos) {
                return Some(world_pos.origin.truncate() + Vec2::new(400., 400.));
            }
        }
        None
    };
    if mouse.just_pressed(MouseButton::Left) {
        let mouse_pos = world_to_board(get_mouse_pos(), 100.);
        if let Some(world_pos) = mouse_pos {
            // Check if a piece exists at this position
            for (_, piece_position, _) in pieces.iter() {
                if piece_position.0 == world_pos {
                    *selected_piece = SelectedPiece(Some(piece_position.clone()));
                    return;
                }
            }
        }
    }
    if mouse.just_pressed(MouseButton::Right) {
        let mouse_pos = world_to_board(get_mouse_pos(), 100.);
        if let Some(world_pos) = mouse_pos {
            if selected_piece.0.is_none() {
                return;
            }
            let selected_piece_info = selected_piece.clone().0.unwrap();
            if selected_piece_info.2 == turn.0 {
                // Change turn
                if validate_piece_move(
                    selected_piece_info.1.clone(),
                    selected_piece_info.2,
                    selected_piece_info.0,
                    world_pos,
                    &pieces,
                ) {
                    turn.0 = match turn.0 {
                        PieceColor::White => PieceColor::Black,
                        PieceColor::Black => PieceColor::White,
                    };
                    move_piece(
                        selected_piece_info.0,
                        world_pos,
                        pieces,
                        commands,
                        selected_piece,
                    );
                }
            }
        } else {
            println!("Mouse is outside the window.");
        }
    }
}
fn move_piece(
    start: UVec2,
    end: UVec2,
    mut pieces: Query<(Entity, &mut PieceInfo, &mut Transform)>,
    mut commands: Commands,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    if start == end {
        return;
    }
    pieces.iter_mut().for_each(|(entity, piece_position, _)| {
        if piece_position.0 == end {
            commands.entity(entity).despawn();
        }
    });
    pieces
        .iter_mut()
        .for_each(|(_, mut piece_position, mut transform)| {
            if piece_position.0 == start {
                // Move piece
                piece_position.0 = end;
                // Update transform for visual position
                transform.translation = Vec3::new(
                    end.x as f32 * 100. + -350., // Adjust for board offset
                    end.y as f32 * 100. + -350.,
                    transform.translation.z,
                );
            }
        });
    selected_piece.0 = None;
}
// Rust
fn world_to_board(world_pos: Option<Vec2>, cell_size: f32) -> Option<UVec2> {
    if let Some(world_pos) = world_pos {
        if world_pos.x < 0.0 || world_pos.y < 0.0 {
            return None; // Outside board
        }
        let col = (world_pos.x / cell_size).floor() as u32;
        let row = (world_pos.y / cell_size).floor() as u32;
        Some(UVec2::new(col, row))
    } else {
        None
    }
}
fn validate_piece_move(piece_type: PieceType, piece_color: PieceColor, start: UVec2, end: UVec2, _pieces: &Query<(Entity, &mut PieceInfo, &mut Transform)>) -> bool {
    let delta: IVec2 = end.as_ivec2() - start.as_ivec2();
    match piece_type {
        PieceType::Pawn => {
            // Pawns move forward by 1
            // TODO: Add capturing, promotion and en passant logic
            delta.x == 0 && delta.y == 1 * match piece_color {
                PieceColor::White => 1,
                PieceColor::Black => -1,
            } || start.y == match piece_color {
                PieceColor::White => 1,
                PieceColor::Black => 6,
            } && delta.x == 0 && delta.y == 2 * match piece_color {
                PieceColor::White => 1,
                PieceColor::Black => -1,
            }

        }
        
        PieceType::King => {
            // Kings move one square in any direction
            //TODO: Add castling logic
            delta.x.abs() <= 1 && delta.y.abs() <= 1
        }
        PieceType::Knight => {
            // Knights move in L-shapes
            (delta.x.abs() == 2 && delta.y.abs() == 1) || (delta.x.abs() == 1 && delta.y.abs() == 2)
        }
        // TODO: Implement path checking for Rook, Bishop, and Queen to ensure no pieces are blocking the path
        PieceType::Rook => {
            // Rooks move in straight lines
            delta.x == 0 || delta.y == 0
        }
        PieceType::Bishop => {
            // Bishops move diagonally
            delta.x.abs() == delta.y.abs()
        }
        PieceType::Queen => {
            // Queens move in straight lines or diagonally
            delta.x == 0 || delta.y == 0 || delta.x.abs() == delta.y.abs()
        }
    }
}
