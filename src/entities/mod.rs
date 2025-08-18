pub mod player;
pub mod enemy;
pub mod collectible;

use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            collectible::CollectiblePlugin,
        ));
    }
}
