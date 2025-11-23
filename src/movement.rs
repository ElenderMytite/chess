use bevy::prelude::*;

use crate::pieces::PieceInfo;
#[derive(Resource, Clone)]
pub struct SelectedPiece(pub Option<UVec2>);

pub fn selection(
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
                    selected_piece.0 = Some(world_pos);
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
            move_piece(
                selected_piece.clone().0.unwrap(),
                world_pos,
                pieces,
                commands,
                selected_piece,
            );
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
