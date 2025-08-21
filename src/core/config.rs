use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    // Player settings
    pub player_base_speed: f32,
    pub player_base_health: i32,
    pub player_base_damage: i32,
    
    // Enemy settings
    pub enemy_spawn_rate: f32,
    pub enemy_difficulty_scaling: f32,
    pub boss_spawn_time: f32,
    
    // World settings
    pub tile_size: f32,
    pub chunk_size: u32,
    
    // UI settings
    pub show_health_bars: bool,
    pub show_damage_numbers: bool,
    pub show_minimap: bool,
    
    // Audio settings
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    
    // Graphics settings
    pub particle_effects: bool,
    pub screen_shake: bool,
    pub vsync: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            player_base_speed: 200.0,
            player_base_health: 100,
            player_base_damage: 10,
            enemy_spawn_rate: 3.0,
            enemy_difficulty_scaling: 1.1,
            boss_spawn_time: 600.0,
            tile_size: 32.0,
            chunk_size: 16,
            show_health_bars: true,
            show_damage_numbers: true,
            show_minimap: true,
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            particle_effects: true,
            screen_shake: true,
            vsync: true,
        }
    }
}

impl GameConfig {
    pub fn load() -> Self {
        // Try to load from file, otherwise use defaults
        if let Ok(contents) = std::fs::read_to_string("config.json") {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }
    
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write("config.json", json);
        }
    }
}
