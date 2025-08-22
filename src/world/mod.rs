pub mod tilemap;
pub mod level_loader;
pub mod tile_animator;

use bevy::prelude::*;
use crate::core::state::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // default mapping (char -> tile def)
            .init_resource::<tilemap::TilemapConfig>()
            // spawn the test level right when we enter Playing
            .add_systems(OnEnter(GameState::Playing), level_loader::load_test_level)
            // animate tiles while Playing
            .add_systems(Update, tile_animator::animate_tiles.run_if(in_state(GameState::Playing)))
            // clean up level when leaving Playing
            .add_systems(OnExit(GameState::Playing), level_loader::despawn_level);
    }
}
