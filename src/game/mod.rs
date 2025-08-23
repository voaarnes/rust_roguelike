// Game module - Contains all gameplay-related systems and components

pub mod player;          // Player entity and movement systems
pub mod enemy;           // Enemy AI and spawning
pub mod collectible;     // Fruit and coin collection
pub mod combat;          // Combat mechanics and damage
pub mod movement;        // Movement utilities
pub mod spawning;        // Wave-based enemy spawning
pub mod progression;     // Level progression system
pub mod abilities;       // Player abilities (future expansion)
pub mod items;           // Item system (future expansion)
pub mod animation;       // Animation controller
pub mod audio;           // Game audio systems
pub mod player_visual;   // Player visual customization based on fruits

use bevy::prelude::*;

/// Plugin that registers all game-related systems
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

