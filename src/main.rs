use bevy::{prelude::*, window::WindowResolution};
use movement::SelectedPiece;
mod pieces;
mod board;
mod atlas;
mod movement;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800,800),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(SelectedPiece(None))
    .add_systems(Startup, board::setup_board_and_camera)
    .add_systems(Startup, (atlas::load_atlas, pieces::setup_pieces).chain())
    .add_systems(Update, movement::selection)
    .run();
}