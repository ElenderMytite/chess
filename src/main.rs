use bevy::prelude::*;
use board::setup_board_and_camera;
use pieces::setup_pieces;
mod pieces;
mod board;
mod atlas;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup_board_and_camera)
    .add_systems(Startup, (atlas::load_atlas, setup_pieces).chain())
    .run();
}