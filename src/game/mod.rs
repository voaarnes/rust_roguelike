pub mod player;
pub mod enemy;
pub mod collectible;
pub mod combat;
pub mod movement;
pub mod spawning;
pub mod progression;
pub mod abilities;
pub mod items;
pub mod animation;
pub mod audio;
pub mod player_visual;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                player::PlayerPlugin,
                player_visual::PlayerVisualPlugin,
                enemy::EnemyPlugin,
                collectible::CollectiblePlugin,
                combat::CombatPlugin,
                movement::MovementPlugin,
                spawning::SpawningPlugin,
                progression::ProgressionPlugin,
                abilities::AbilitiesPlugin,
                items::ItemsPlugin,
                animation::AnimationPlugin,
                audio::AudioPlugin,
            ));
    }
}

