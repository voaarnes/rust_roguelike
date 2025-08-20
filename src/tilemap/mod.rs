pub mod tilemap;
pub mod tile_loader;

use bevy::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(tilemap::TilemapConfig::default())
            .add_systems(Startup, tile_loader::load_test_level);
    }
}
