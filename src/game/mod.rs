use bevy::prelude::*;
use crate::world::WorldPlugin;

pub mod player;
pub mod enemy;
pub mod combat;
pub mod movement;
pub mod spawning;
pub mod progression;
pub mod abilities;
pub mod items;
pub mod animation;
pub mod audio;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                enemy::EnemyPlugin,
                combat::CombatPlugin,
                movement::MovementPlugin,
               player::PlayerPlugin,
                spawning::SpawningPlugin,
                progression::ProgressionPlugin,
                abilities::AbilitiesPlugin,
                items::ItemsPlugin,
                animation::AnimationPlugin,
                audio::AudioPlugin,
                WorldPlugin,
            ));
    }
}
