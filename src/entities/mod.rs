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

fn spawn_initial_collectibles(
    mut commands: Commands,
    atlases: Res<collectible::FruitAtlases>,
) {
    // put 1 of each fruit on screen so you can see them
    collectible::spawn_collectible(&mut commands, &atlases, Vec3::new(-64.0, 0.0, 2.0), collectible::CollectibleType::Strawberry);
    collectible::spawn_collectible(&mut commands, &atlases, Vec3::new(  0.0, 0.0, 2.0), collectible::CollectibleType::Pear);
    collectible::spawn_collectible(&mut commands, &atlases, Vec3::new( 64.0, 0.0, 2.0), collectible::CollectibleType::Mango);
}
