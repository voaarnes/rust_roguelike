use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayState {
    #[default]
    Exploring,
    Combat,
    Shopping,
    Dialogue,
    Transitioning,
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
