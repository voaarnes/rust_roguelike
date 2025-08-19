pub mod tilemap;
pub mod tile_loader;

use bevy::prelude::*;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<tilemap::TilemapConfig>()
            .add_systems(Startup, tile_loader::load_test_level.in_set(TilemapSet::LoadLevel));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TilemapSet {
    LoadLevel,
}
