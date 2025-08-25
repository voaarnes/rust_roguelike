use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct SaveData {
    pub player_level: u32,
    pub player_experience: u32,
    pub unlocked_abilities: Vec<String>,
    pub completed_levels: Vec<usize>,
    pub total_play_time: f32,
    pub high_score: u32,
    pub achievements: Vec<String>,
    
    // New systems data
    pub currency: CurrencyData,
    pub talents: TalentData,
    pub achievement_progress: HashMap<String, u32>,
    pub unlocked_achievements: Vec<String>,
    pub shop_purchases: Vec<String>,
    pub prestige_level: u32,
    pub prestige_points: u32,
    pub total_combo: u32,
    pub max_combo: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CurrencyData {
    pub coins: u32,
    pub gems: u32,
    pub soul_shards: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TalentData {
    pub available_points: u32,
    pub unlocked_talents: HashMap<String, u32>,
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

pub fn save_game_state(
    player_q: Query<&crate::game::player::Player>,
    currency: Res<crate::systems::shop::PlayerCurrency>,
    talents: Res<crate::systems::talents::PlayerTalents>,
    achievements: Res<crate::systems::achievements::PlayerAchievements>,
    combo_tracker: Res<crate::systems::combo::ComboTracker>,
    prestige: Res<crate::systems::prestige::PrestigeSystem>,
    mut save_data: ResMut<SaveData>,
) {
    if let Ok(player) = player_q.single() {
        save_data.player_level = player.level;
        save_data.player_experience = player.experience;
    }
    
    // Save currency
    save_data.currency = CurrencyData {
        coins: currency.coins,
        gems: currency.gems,
        soul_shards: currency.soul_shards,
    };
    
    // Save talents
    save_data.talents = TalentData {
        available_points: talents.available_points,
        unlocked_talents: talents.unlocked_talents.clone(),
    };
    
    // Save achievements
    save_data.achievement_progress = achievements.progress.clone();
    save_data.unlocked_achievements = achievements.unlocked.keys().cloned().collect();
    
    // Save combo stats
    save_data.total_combo = combo_tracker.total_combo_points as u32;
    save_data.max_combo = combo_tracker.max_combo;
    
    // Save prestige
    save_data.prestige_level = prestige.current_prestige;
    save_data.prestige_points = prestige.prestige_points;
}

pub fn load_game_state(
    save_data: Res<SaveData>,
    mut currency: ResMut<crate::systems::shop::PlayerCurrency>,
    mut talents: ResMut<crate::systems::talents::PlayerTalents>,
    mut achievements: ResMut<crate::systems::achievements::PlayerAchievements>,
    mut combo_tracker: ResMut<crate::systems::combo::ComboTracker>,
    mut prestige: ResMut<crate::systems::prestige::PrestigeSystem>,
) {
    // Load currency
    currency.coins = save_data.currency.coins;
    currency.gems = save_data.currency.gems;
    currency.soul_shards = save_data.currency.soul_shards;
    
    // Load talents
    talents.available_points = save_data.talents.available_points;
    talents.unlocked_talents = save_data.talents.unlocked_talents.clone();
    
    // Load achievements
    achievements.progress = save_data.achievement_progress.clone();
    for achievement_id in &save_data.unlocked_achievements {
        achievements.unlocked.insert(achievement_id.clone(), true);
    }
    
    // Load combo stats
    combo_tracker.total_combo_points = save_data.total_combo as u64;
    combo_tracker.max_combo = save_data.max_combo;
    
    // Load prestige
    prestige.current_prestige = save_data.prestige_level;
    prestige.prestige_points = save_data.prestige_points;
}
