use bevy::prelude::*;
use std::collections::HashMap;

pub struct PrestigePlugin;

impl Plugin for PrestigePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PrestigeSystem>()
            .init_resource::<MetaProgression>()
            .add_event::<PrestigeEvent>()
            .add_systems(Update, check_prestige_eligibility)
            .add_systems(Update, handle_prestige)
            .add_systems(Update, apply_meta_bonuses);
    }
}

#[derive(Resource)]
pub struct PrestigeSystem {
    pub current_prestige: u32,
    pub total_prestiges: u32,
    pub prestige_points: u32,
    pub legacy_points: u32,
    pub ascension_shards: u32,
}

#[derive(Resource)]
pub struct MetaProgression {
    pub permanent_upgrades: HashMap<String, MetaUpgrade>,
    pub unlocked_features: Vec<String>,
    pub starting_bonuses: StartingBonuses,
    pub milestone_rewards: HashMap<u32, MilestoneReward>,
}

#[derive(Clone)]
pub struct MetaUpgrade {
    pub id: String,
    pub name: String,
    pub description: String,
    pub current_level: u32,
    pub max_level: u32,
    pub cost_formula: CostFormula,
    pub effect: MetaEffect,
}

#[derive(Clone)]
pub enum CostFormula {
    Linear(u32),           // base cost per level
    Exponential(f32, u32), // multiplier, base
    Custom(Vec<u32>),      // specific costs per level
}

#[derive(Clone)]
pub enum MetaEffect {
    StartingGold(u32),
    StartingLevel(u32),
    PermanentStatBoost(StatType, f32),
    UnlockStartingItem(String),
    ExperienceMultiplier(f32),
    LuckBonus(f32),
    StartingTalentPoints(u32),
    ReviveTokens(u32),
}

#[derive(Clone, Copy)]
pub enum StatType {
    Health,
    Damage,
    Speed,
    CritChance,
    DropRate,
}

#[derive(Clone, Default)]
pub struct StartingBonuses {
    pub gold: u32,
    pub level: u32,
    pub items: Vec<String>,
    pub talent_points: u32,
    pub stat_bonuses: HashMap<StatType, f32>,
}

#[derive(Clone)]
pub struct MilestoneReward {
    pub description: String,
    pub rewards: Vec<RewardType>,
}

#[derive(Clone)]
pub enum RewardType {
    Currency(CurrencyType, u32),
    PermanentUpgrade(String),
    CosmeticUnlock(String),
    FeatureUnlock(String),
    TitleUnlock(String),
}

#[derive(Clone, Copy)]
pub enum CurrencyType {
    PrestigePoints,
    LegacyPoints,
    AscensionShards,
}

#[derive(Event)]
pub struct PrestigeEvent {
    pub prestige_type: PrestigeType,
    pub player: Entity,
}

#[derive(Clone, Copy)]
pub enum PrestigeType {
    Standard,    // Reset progress for prestige points
    Ascension,   // Harder reset for ascension shards
    Rebirth,     // Complete reset for legacy points
}

impl Default for PrestigeSystem {
    fn default() -> Self {
        Self {
            current_prestige: 0,
            total_prestiges: 0,
            prestige_points: 0,
            legacy_points: 0,
            ascension_shards: 0,
        }
    }
}

impl Default for MetaProgression {
    fn default() -> Self {
        let mut permanent_upgrades = HashMap::new();
        
        // Define meta upgrades
        permanent_upgrades.insert("starting_gold".to_string(), MetaUpgrade {
            id: "starting_gold".to_string(),
            name: "Golden Start".to_string(),
            description: "Start each run with bonus gold".to_string(),
            current_level: 0,
            max_level: 10,
            cost_formula: CostFormula::Linear(1),
            effect: MetaEffect::StartingGold(50),
        });
        
        permanent_upgrades.insert("health_boost".to_string(), MetaUpgrade {
            id: "health_boost".to_string(),
            name: "Eternal Vitality".to_string(),
            description: "+5% permanent health bonus".to_string(),
            current_level: 0,
            max_level: 20,
            cost_formula: CostFormula::Exponential(1.5, 2),
            effect: MetaEffect::PermanentStatBoost(StatType::Health, 0.05),
        });
        
        permanent_upgrades.insert("exp_multiplier".to_string(), MetaUpgrade {
            id: "exp_multiplier".to_string(),
            name: "Wisdom of Ages".to_string(),
            description: "+10% experience gain".to_string(),
            current_level: 0,
            max_level: 5,
            cost_formula: CostFormula::Custom(vec![5, 10, 20, 40, 80]),
            effect: MetaEffect::ExperienceMultiplier(0.1),
        });
        
        permanent_upgrades.insert("luck_boost".to_string(), MetaUpgrade {
            id: "luck_boost".to_string(),
            name: "Fortune's Favor".to_string(),
            description: "+5% better loot drops".to_string(),
            current_level: 0,
            max_level: 10,
            cost_formula: CostFormula::Linear(3),
            effect: MetaEffect::LuckBonus(0.05),
        });
        
        // Define milestones
        let mut milestone_rewards = HashMap::new();
        
        milestone_rewards.insert(1, MilestoneReward {
            description: "First Prestige".to_string(),
            rewards: vec![
                RewardType::Currency(CurrencyType::PrestigePoints, 10),
                RewardType::TitleUnlock("Reborn".to_string()),
            ],
        });
        
        milestone_rewards.insert(5, MilestoneReward {
            description: "Prestige Veteran".to_string(),
            rewards: vec![
                RewardType::Currency(CurrencyType::LegacyPoints, 1),
                RewardType::FeatureUnlock("hardcore_mode".to_string()),
            ],
        });
        
        milestone_rewards.insert(10, MilestoneReward {
            description: "Prestige Master".to_string(),
            rewards: vec![
                RewardType::Currency(CurrencyType::AscensionShards, 1),
                RewardType::CosmeticUnlock("golden_aura".to_string()),
                RewardType::PermanentUpgrade("auto_prestige".to_string()),
            ],
        });
        
        Self {
            permanent_upgrades,
            unlocked_features: Vec::new(),
            starting_bonuses: StartingBonuses::default(),
            milestone_rewards,
        }
    }
}

fn check_prestige_eligibility(
    _prestige: Res<PrestigeSystem>,
    wave_manager: Res<crate::game::spawning::WaveManager>,
    player_q: Query<&crate::game::player::Player>,
) {
    if let Ok(player) = player_q.single() {
        // Standard prestige available at wave 50 or level 100
        let eligible = wave_manager.current_wave >= 50 || player.level >= 100;
        if eligible {
            println!("Prestige is now available!");
        }
    }
}

fn handle_prestige(
    mut events: EventReader<PrestigeEvent>,
    mut prestige: ResMut<PrestigeSystem>,
    mut meta: ResMut<MetaProgression>,
    mut player_q: Query<&mut crate::game::player::Player>,
    wave_manager: Res<crate::game::spawning::WaveManager>,
) {
    for event in events.read() {
        match event.prestige_type {
            PrestigeType::Standard => {
                // Calculate prestige points based on progress
                let points = calculate_prestige_points(
                    wave_manager.current_wave,
                if let Ok(player) = player_q.single() {
                    player.level
                } else {
                    1
                },
                    prestige.current_prestige,
                );
                
                prestige.prestige_points += points;
                prestige.current_prestige += 1;
                prestige.total_prestiges += 1;
                
                // Check for milestone rewards
                if let Some(milestone) = meta.milestone_rewards.get(&prestige.total_prestiges).cloned() {
                    println!("Milestone reached: {}", milestone.description);
                    apply_milestone_rewards(&milestone.rewards, &mut prestige, &mut meta);
                }
                
                // Reset player progress
                if let Ok(mut player) = player_q.single_mut() {
                    player.level = 1 + meta.starting_bonuses.level;
                    player.experience = 0;
                }
                
                println!("Prestige complete! Gained {} prestige points", points);
            }
            PrestigeType::Ascension => {
                // Harder reset with better rewards
                let shards = prestige.current_prestige / 10;
                prestige.ascension_shards += shards;
                prestige.current_prestige = 0;
                
                println!("Ascension complete! Gained {} ascension shards", shards);
            }
            PrestigeType::Rebirth => {
                // Complete reset for legacy points
                let legacy = prestige.total_prestiges / 5;
                prestige.legacy_points += legacy;
                prestige.total_prestiges = 0;
                prestige.current_prestige = 0;
                
                println!("Rebirth complete! Gained {} legacy points", legacy);
            }
        }
    }
}

fn calculate_prestige_points(wave: u32, level: u32, current_prestige: u32) -> u32 {
    let base_points = wave + level;
    let prestige_multiplier = 1.0 + (current_prestige as f32 * 0.1);
    (base_points as f32 * prestige_multiplier) as u32
}

fn apply_milestone_rewards(
    rewards: &[RewardType],
    prestige: &mut PrestigeSystem,
    meta: &mut MetaProgression,
) {
    for reward in rewards {
        match reward {
            RewardType::Currency(currency_type, amount) => {
                match currency_type {
                    CurrencyType::PrestigePoints => prestige.prestige_points += amount,
                    CurrencyType::LegacyPoints => prestige.legacy_points += amount,
                    CurrencyType::AscensionShards => prestige.ascension_shards += amount,
                }
            }
            RewardType::FeatureUnlock(feature) => {
                meta.unlocked_features.push(feature.clone());
            }
            _ => {}
        }
    }
}

fn apply_meta_bonuses(
    meta: Res<MetaProgression>,
    mut player_q: Query<(&mut crate::game::combat::CombatStats, &mut crate::game::player::Player), Added<crate::game::player::Player>>,
    mut currency: ResMut<crate::systems::shop::PlayerCurrency>,
) {
    if let Ok((mut stats, mut player)) = player_q.single_mut() {
        // Apply starting bonuses from meta progression
        currency.coins += meta.starting_bonuses.gold;
        player.level = 1 + meta.starting_bonuses.level;
        
        // Apply permanent stat bonuses
        for (stat_type, bonus) in &meta.starting_bonuses.stat_bonuses {
            match stat_type {
                StatType::Health => {
                    // Apply health bonus
                }
                StatType::Damage => {
                    stats.damage = (stats.damage as f32 * (1.0 + bonus)) as i32;
                }
                _ => {}
            }
        }
    }
}

fn calculate_prestige_currency(
    prestige: Res<PrestigeSystem>,
    game_stats: Res<crate::core::state::GameStats>,
) -> u32 {
    // Calculate how much prestige currency would be earned
    let base = game_stats.enemies_killed + game_stats.coins_collected / 10;
    let multiplier = 1.0 + (prestige.current_prestige as f32 * 0.05);
    (base as f32 * multiplier) as u32
}
