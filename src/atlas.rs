use bevy::prelude::*;
#[derive(Resource)]
pub struct PieceAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn load_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("pieces2.png");

    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(200), // one cell = 32×32
        6,                 // columns
        2,                 // rows
        None,
        None,
    ));

    commands.insert_resource(PieceAtlas { texture, layout });
}