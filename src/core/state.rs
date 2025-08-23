// Core game state management

use bevy::prelude::*;

/// Main game states that control the overall flow of the application
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,    // Title screen and menu
    Playing,     // Active gameplay
    Paused,      // Game paused
    GameOver,    // Player died
    Victory,     // Player won (future use)
}

/// Sub-states within the Playing state for more granular control
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayState {
    #[default]
    Exploring,      // Normal gameplay
    Combat,         // Combat encounters (future use)
    Shopping,       // Item shop (future use)
    Dialogue,       // NPC dialogue (future use)
    Transitioning,  // Level transitions
}

#[derive(Resource)]
pub struct GameStats {
    pub current_level: usize,
    pub score: u32,
    pub enemies_killed: u32,
    pub time_played: f32,
    pub coins_collected: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub distance_traveled: f32,
    pub abilities_used: u32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            current_level: 1,
            score: 0,
            enemies_killed: 0,
            time_played: 0.0,
            coins_collected: 0,
            damage_dealt: 0,
            damage_taken: 0,
            distance_traveled: 0.0,
            abilities_used: 0,
        }
    }
}
