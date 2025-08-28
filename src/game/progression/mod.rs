use bevy::prelude::*;
use crate::core::state::{GameState, GameStats};
use crate::game::spawning::WaveManager;
use crate::systems::talents::{PlayerTalents, UnlockTalentEvent};
use crate::systems::achievements::AchievementUnlockedEvent;
use crate::systems::prestige::{PrestigeSystem, PrestigeEvent};
use crate::systems::quests::QuestCompleteEvent;

pub struct ProgressionPlugin;

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameStats>()
            .init_resource::<ExperienceTracker>()
            .add_systems(Update, (
                handle_experience,
                check_level_progression,
                check_prestige_eligibility,
                award_talent_points,
            ));
    }
}

#[derive(Resource, Default)]
pub struct ExperienceTracker {
    pub current_xp: u32,
    pub xp_to_next_level: u32,
    pub current_level: u32,
    pub talent_points: u32,
}

impl ExperienceTracker {
    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.current_xp += amount;
        if self.current_xp >= self.xp_to_next_level {
            self.level_up();
            return true;
        }
        false
    }
    
    fn level_up(&mut self) {
        self.current_level += 1;
        self.talent_points += 1;
        self.current_xp = 0;
        self.xp_to_next_level = self.calculate_xp_required(self.current_level + 1);
    }
    
    fn calculate_xp_required(&self, level: u32) -> u32 {
        // Exponential growth: level^1.5 * 100
        ((level as f32).powf(1.5) * 100.0) as u32
    }
}

fn handle_experience(
    mut xp_tracker: ResMut<ExperienceTracker>,
    game_stats: Res<GameStats>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
    player_q: Query<Entity, With<crate::game::player::Player>>,
) {
    let Ok(player_entity) = player_q.get_single() else { return };
    
    // Award XP based on game progress
    let xp_from_kills = game_stats.enemies_killed * 10;
    let xp_from_coins = game_stats.coins_collected / 10;
    let total_xp = xp_from_kills + xp_from_coins;
    
    if total_xp > xp_tracker.current_xp {
        let gained_xp = total_xp - xp_tracker.current_xp;
        if xp_tracker.add_xp(gained_xp) {
            // Level up occurred
            achievement_events.write(AchievementUnlockedEvent {
                achievement_id: format!("level_{}", xp_tracker.current_level),
                player: player_entity,
            });
            
            if xp_tracker.current_level == 10 {
                achievement_events.write(AchievementUnlockedEvent {
                    achievement_id: "level_master".to_string(),
                    player: player_entity,
                });
            }
        }
    }
}

/// System to check if player should progress to the next level
fn check_level_progression(
    wave_manager: Res<WaveManager>,
    mut game_stats: ResMut<GameStats>,
    _next_state: ResMut<NextState<GameState>>,
    mut achievement_events: EventWriter<AchievementUnlockedEvent>,
    mut quest_events: EventWriter<QuestCompleteEvent>,
    player_q: Query<Entity, With<crate::game::player::Player>>,
) {
    let Ok(player_entity) = player_q.get_single() else { return };
    
    // Progress to next level after defeating boss waves (every 5th wave)
    if wave_manager.current_wave > 0 && wave_manager.current_wave % 5 == 0 && wave_manager.wave_complete {
        game_stats.current_level += 1;
        
        // Trigger wave completion achievements
        achievement_events.write(AchievementUnlockedEvent {
            achievement_id: format!("wave_{}", wave_manager.current_wave),
            player: player_entity,
        });
        
        // Trigger wave quest completion
        quest_events.write(QuestCompleteEvent {
            quest_id: "daily_survivor".to_string(),
            player: player_entity,
        });
        
        if wave_manager.current_wave == 50 {
            achievement_events.write(AchievementUnlockedEvent {
                achievement_id: "wave_master".to_string(),
                player: player_entity,
            });
        }
        
        // Could transition to a level selection screen or continue
        // For now, just continue playing
    }
}

fn check_prestige_eligibility(
    game_stats: Res<GameStats>,
    wave_manager: Res<WaveManager>,
    mut prestige_system: ResMut<PrestigeSystem>,
    mut prestige_events: EventWriter<PrestigeEvent>,
    player_q: Query<Entity, With<crate::game::player::Player>>,
) {
    let Ok(player_entity) = player_q.get_single() else { return };
    
    // Check if player is eligible for prestige (e.g., reached wave 100)
    if wave_manager.current_wave >= 100 && prestige_system.current_prestige == 0 {
        prestige_events.write(PrestigeEvent {
            prestige_type: crate::systems::prestige::PrestigeType::Standard,
            player: player_entity,
        });
    }
}

fn award_talent_points(
    mut xp_tracker: ResMut<ExperienceTracker>,
    mut player_talents: ResMut<PlayerTalents>,
    mut talent_events: EventWriter<UnlockTalentEvent>,
) {
    if xp_tracker.talent_points > 0 {
        player_talents.available_points += xp_tracker.talent_points;
        xp_tracker.talent_points = 0;
        
        // Auto-suggest talent unlock (could be player choice later)
        if player_talents.available_points >= 1 {
            talent_events.write(UnlockTalentEvent {
                talent_id: "basic_damage".to_string(),
                tree_type: crate::systems::talents::TalentTreeType::Offense,
            });
        }
    }
}
