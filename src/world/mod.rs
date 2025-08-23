pub mod tilemap;
pub mod level_loader;
pub mod collision;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                tilemap::TilemapPlugin,
                collision::TileCollisionPlugin,
            ));
    }
}
