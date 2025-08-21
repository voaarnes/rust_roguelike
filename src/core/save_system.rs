use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct SaveData {
    pub player_level: u32,
    pub player_experience: u32,
    pub unlocked_abilities: Vec<String>,
    pub completed_levels: Vec<usize>,
    pub total_play_time: f32,
    pub high_score: u32,
    pub achievements: Vec<String>,
}

impl SaveData {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("save.json", json)?;
        Ok(())
    }
    
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("save.json")?;
        let data = serde_json::from_str(&contents)?;
        Ok(data)
    }
}

pub fn auto_save_system(
    save_data: Res<SaveData>,
    time: Res<Time>,
    mut last_save: Local<f32>,
) {
    let current_time = time.elapsed_secs();
    if current_time - *last_save > 60.0 { // Auto-save every minute
        if let Err(e) = save_data.save() {
            warn!("Failed to auto-save: {}", e);
        } else {
            info!("Game auto-saved");
        }
        *last_save = current_time;
    }
}
