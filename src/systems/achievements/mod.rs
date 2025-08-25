use bevy::prelude::*;
use std::collections::HashMap;

pub struct AchievementPlugin;

impl Plugin for AchievementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AchievementRegistry>()
            .init_resource::<PlayerAchievements>()
            .add_event::<AchievementUnlockedEvent>()
            .add_systems(Startup, initialize_achievements)
            .add_systems(Update, (
                track_achievement_progress,
                check_achievement_completion,
                handle_achievement_rewards,
            ));
    }
}

#[derive(Resource, Default)]
pub struct AchievementRegistry {
    pub achievements: HashMap<String, Achievement>,
}

#[derive(Clone)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub category: AchievementCategory,
    pub requirement: AchievementRequirement,
    pub reward: AchievementReward,
    pub hidden: bool,
    pub tier: AchievementTier,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AchievementCategory {
    Combat,
    Collection,
    Progression,
    Exploration,
    Challenge,
    Secret,
}

#[derive(Clone)]
pub enum AchievementRequirement {
    KillEnemies(u32),
    CollectCoins(u32),
    ReachWave(u32),
    DefeatBoss(String),
    CompleteWithoutDamage(u32), // waves
    UseAbility(String, u32), // ability name, count
    CollectFruits(u32),
    ReachCombo(u32),
    CompleteChallenge(String),
    Custom(String, u32, u32), // stat name, required value, current value
}

#[derive(Clone)]
pub struct AchievementReward {
    pub currency: Option<(CurrencyType, u32)>,
    pub unlock: Option<String>,
    pub title: Option<String>,
    pub cosmetic: Option<String>,
    pub bonus_stats: Option<Vec<(StatType, f32)>>,
}

#[derive(Clone, Copy)]
pub enum AchievementTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[derive(Clone, Copy)]
pub enum CurrencyType {
    Coins,
    Gems,
    SoulShards,
}

#[derive(Clone, Copy)]
pub enum StatType {
    Health,
    Damage,
    Speed,
    ExperienceGain,
}

#[derive(Resource)]
pub struct PlayerAchievements {
    pub unlocked: HashMap<String, bool>,
    pub progress: HashMap<String, u32>,
    pub total_points: u32,
}

#[derive(Event)]
pub struct AchievementUnlockedEvent {
    pub achievement_id: String,
    pub player: Entity,
}

impl Default for PlayerAchievements {
    fn default() -> Self {
        Self {
            unlocked: HashMap::new(),
            progress: HashMap::new(),
            total_points: 0,
        }
    }
}

fn initialize_achievements(mut registry: ResMut<AchievementRegistry>) {
    let achievements = vec![
        Achievement {
            id: "first_blood".to_string(),
            name: "First Blood".to_string(),
            description: "Defeat your first enemy".to_string(),
            icon: "icons/sword.png".to_string(),
            category: AchievementCategory::Combat,
            requirement: AchievementRequirement::KillEnemies(1),
            reward: AchievementReward {
                currency: Some((CurrencyType::Coins, 50)),
                unlock: None,
                title: Some("Novice Warrior".to_string()),
                cosmetic: None,
                bonus_stats: None,
            },
            hidden: false,
            tier: AchievementTier::Bronze,
        },
        Achievement {
            id: "wave_10".to_string(),
            name: "Survivor".to_string(),
            description: "Reach wave 10".to_string(),
            icon: "icons/shield.png".to_string(),
            category: AchievementCategory::Progression,
            requirement: AchievementRequirement::ReachWave(10),
            reward: AchievementReward {
                currency: Some((CurrencyType::Gems, 10)),
                unlock: Some("special_ability_1".to_string()),
                title: Some("Survivor".to_string()),
                cosmetic: None,
                bonus_stats: Some(vec![(StatType::Health, 25.0)]),
            },
            hidden: false,
            tier: AchievementTier::Silver,
        },
        Achievement {
            id: "goblin_slayer".to_string(),
            name: "Goblin Slayer".to_string(),
            description: "Defeat 100 goblins".to_string(),
            icon: "icons/goblin.png".to_string(),
            category: AchievementCategory::Combat,
            requirement: AchievementRequirement::Custom("goblins_killed".to_string(), 100, 0),
            reward: AchievementReward {
                currency: Some((CurrencyType::Coins, 200)),
                unlock: None,
                title: Some("Goblin Slayer".to_string()),
                cosmetic: Some("goblin_slayer_badge".to_string()),
                bonus_stats: Some(vec![(StatType::Damage, 5.0)]),
            },
            hidden: false,
            tier: AchievementTier::Gold,
        },
        Achievement {
            id: "flawless_victory".to_string(),
            name: "Flawless Victory".to_string(),
            description: "Complete 5 waves without taking damage".to_string(),
            icon: "icons/perfect.png".to_string(),
            category: AchievementCategory::Challenge,
            requirement: AchievementRequirement::CompleteWithoutDamage(5),
            reward: AchievementReward {
                currency: Some((CurrencyType::SoulShards, 1)),
                unlock: Some("dodge_ability".to_string()),
                title: Some("Untouchable".to_string()),
                cosmetic: None,
                bonus_stats: Some(vec![(StatType::Speed, 10.0)]),
            },
            hidden: false,
            tier: AchievementTier::Platinum,
        },
        Achievement {
            id: "combo_master".to_string(),
            name: "Combo Master".to_string(),
            description: "Reach a 50x combo".to_string(),
            icon: "icons/combo.png".to_string(),
            category: AchievementCategory::Combat,
            requirement: AchievementRequirement::ReachCombo(50),
            reward: AchievementReward {
                currency: Some((CurrencyType::Gems, 25)),
                unlock: None,
                title: Some("Combo Master".to_string()),
                cosmetic: None,
                bonus_stats: None,
            },
            hidden: false,
            tier: AchievementTier::Gold,
        },
    ];
    
    for achievement in achievements {
        registry.achievements.insert(achievement.id.clone(), achievement);
    }
}

fn track_achievement_progress(
    mut player_achievements: ResMut<PlayerAchievements>,
    registry: Res<AchievementRegistry>,
    game_stats: Res<crate::core::state::GameStats>,
    combo_tracker: Res<crate::systems::combo::ComboTracker>,
    player_stats_q: Query<&crate::game::player::PlayerStats>,
) {
    for (id, achievement) in registry.achievements.iter() {
        if player_achievements.unlocked.get(id).copied().unwrap_or(false) {
            continue;
        }
        
        let progress = match &achievement.requirement {
            AchievementRequirement::KillEnemies(required) => {
                game_stats.enemies_killed.min(*required)
            }
            AchievementRequirement::CollectCoins(required) => {
                game_stats.coins_collected.min(*required)
            }
            AchievementRequirement::ReachWave(required) => {
                (game_stats.current_level as u32).min(*required)
            }
            AchievementRequirement::ReachCombo(required) => {
                combo_tracker.max_combo.min(*required)
            }
            _ => player_achievements.progress.get(id).copied().unwrap_or(0),
        };
        
        player_achievements.progress.insert(id.clone(), progress);
    }
}

fn check_achievement_completion(
    mut player_achievements: ResMut<PlayerAchievements>,
    registry: Res<AchievementRegistry>,
    mut unlock_events: EventWriter<AchievementUnlockedEvent>,
    player_q: Query<Entity, With<crate::game::player::Player>>,
) {
    let Ok(player_entity) = player_q.single() else { return };
    
    for (id, achievement) in registry.achievements.iter() {
        if player_achievements.unlocked.get(id).copied().unwrap_or(false) {
            continue;
        }
        
        let progress = player_achievements.progress.get(id).copied().unwrap_or(0);
        let completed = match &achievement.requirement {
            AchievementRequirement::KillEnemies(required) => progress >= *required,
            AchievementRequirement::CollectCoins(required) => progress >= *required,
            AchievementRequirement::ReachWave(required) => progress >= *required,
            AchievementRequirement::ReachCombo(required) => progress >= *required,
            AchievementRequirement::Custom(_, required, _) => progress >= *required,
            _ => false,
        };
        
        if completed {
            player_achievements.unlocked.insert(id.clone(), true);
            player_achievements.total_points += match achievement.tier {
                AchievementTier::Bronze => 10,
                AchievementTier::Silver => 25,
                AchievementTier::Gold => 50,
                AchievementTier::Platinum => 100,
                AchievementTier::Diamond => 200,
            };
            unlock_events.write(AchievementUnlockedEvent {
                achievement_id: id.clone(),
                player: player_entity,
            });
        }
    }
}

fn handle_achievement_rewards(
    mut events: EventReader<AchievementUnlockedEvent>,
    registry: Res<AchievementRegistry>,
    mut currency: ResMut<crate::systems::shop::PlayerCurrency>,
) {
    for event in events.read() {
        if let Some(achievement) = registry.achievements.get(&event.achievement_id) {
            // Apply rewards
            if let Some((currency_type, amount)) = &achievement.reward.currency {
                match currency_type {
                    CurrencyType::Coins => currency.coins += amount,
                    CurrencyType::Gems => currency.gems += amount,
                    CurrencyType::SoulShards => currency.soul_shards += amount,
                }
            }
            
            println!("Achievement Unlocked: {}", achievement.name);
        }
    }
}
