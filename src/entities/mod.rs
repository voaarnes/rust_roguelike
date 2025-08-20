pub mod player;
pub mod enemy;
pub mod collectible;
pub mod powerup;
pub mod fruit_spawner;

use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            collectible::CollectiblePlugin,
            powerup::PowerUpPlugin,
            fruit_spawner::FruitSpawnerPlugin,
        ));
    }
}
