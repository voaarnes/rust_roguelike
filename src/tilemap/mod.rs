pub mod tilemap;
pub mod level_loader;
pub mod tile_animator;

use bevy::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<tilemap::TilemapConfig>()
            .add_systems(Startup, (
                level_loader::load_test_level,
                apply_deferred,
            ).chain())
            .add_systems(Update, tile_animator::animate_tiles);
    }
}
