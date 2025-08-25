use bevy::prelude::*;
use std::collections::HashMap;

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<QuestManager>()
            .init_resource::<ActiveQuests>()
            .add_event::<QuestCompleteEvent>()
            .add_systems(Startup, initialize_quests)
            .add_systems(Update, (
                update_quest_progress,
                check_quest_completion,
                generate_daily_quests,
                handle_quest_rewards,
            ));
    }
}

#[derive(Resource, Default)]
pub struct QuestManager {
    pub available_quests: HashMap<String, Quest>,
    pub daily_quest_pool: Vec<String>,
    pub wave_challenges: HashMap<u32, Vec<String>>,
}

#[derive(Clone)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quest_type: QuestType,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
    pub time_limit: Option<f32>,
    pub repeatable: bool,
}

#[derive(Clone, Copy)]
pub enum QuestType {
    Daily,
    Wave,
    Story,
    Hidden,
    Challenge,
}

#[derive(Clone)]
pub enum QuestObjective {
    KillEnemies(String, u32), // enemy type, count
    CollectItems(String, u32), // item type, count
    ReachWave(u32),
    SurviveTime(f32),
    UseAbility(String, u32), // ability name, times
    CompleteWithoutDamage,
    ReachCombo(u32),
    DefeatBossInTime(String, f32), // boss name, time limit
    Custom(String, u32), // custom objective, target value
}

#[derive(Clone)]
pub struct QuestRewards {
    pub experience: u32,
    pub currency: Vec<(CurrencyType, u32)>,
    pub items: Vec<String>,
    pub unlock: Option<String>,
}

#[derive(Clone, Copy)]
pub enum CurrencyType {
    Coins,
    Gems,
    SoulShards,
}

#[derive(Resource)]
pub struct ActiveQuests {
    pub daily_quests: Vec<ActiveQuest>,
    pub wave_challenges: Vec<ActiveQuest>,
    pub story_quests: Vec<ActiveQuest>,
    pub completed_quests: HashMap<String, u32>, // quest_id -> times completed
}

#[derive(Clone)]
pub struct ActiveQuest {
    pub quest: Quest,
    pub progress: HashMap<usize, u32>, // objective index -> current progress
    pub start_time: f32,
    pub completed: bool,
}

#[derive(Event)]
pub struct QuestCompleteEvent {
    pub quest_id: String,
    pub player: Entity,
}

impl Default for ActiveQuests {
    fn default() -> Self {
        Self {
            daily_quests: Vec::new(),
            wave_challenges: Vec::new(),
            story_quests: Vec::new(),
            completed_quests: HashMap::new(),
        }
    }
}

fn initialize_quests(mut quest_manager: ResMut<QuestManager>) {
    // Daily quest pool
    let daily_quests = vec![
        Quest {
            id: "daily_slayer".to_string(),
            name: "Monster Slayer".to_string(),
            description: "Defeat 50 enemies".to_string(),
            quest_type: QuestType::Daily,
            objectives: vec![QuestObjective::KillEnemies("any".to_string(), 50)],
            rewards: QuestRewards {
                experience: 100,
                currency: vec![(CurrencyType::Coins, 200)],
                items: vec![],
                unlock: None,
            },
            time_limit: Some(86400.0), // 24 hours
            repeatable: true,
        },
        Quest {
            id: "daily_collector".to_string(),
            name: "Treasure Hunter".to_string(),
            description: "Collect 100 coins".to_string(),
            quest_type: QuestType::Daily,
            objectives: vec![QuestObjective::CollectItems("coin".to_string(), 100)],
            rewards: QuestRewards {
                experience: 50,
                currency: vec![(CurrencyType::Gems, 5)],
                items: vec![],
                unlock: None,
            },
            time_limit: Some(86400.0),
            repeatable: true,
        },
        Quest {
            id: "daily_survivor".to_string(),
            name: "Survivor".to_string(),
            description: "Reach wave 10 without dying".to_string(),
            quest_type: QuestType::Daily,
            objectives: vec![QuestObjective::ReachWave(10)],
            rewards: QuestRewards {
                experience: 200,
                currency: vec![(CurrencyType::Coins, 500)],
                items: vec!["health_potion".to_string()],
                unlock: None,
            },
            time_limit: Some(86400.0),
            repeatable: true,
        },
    ];
    
    for quest in daily_quests {
        quest_manager.available_quests.insert(quest.id.clone(), quest.clone());
        quest_manager.daily_quest_pool.push(quest.id);
    }
    
    // Wave challenges
    let wave_5_challenge = Quest {
        id: "wave_5_flawless".to_string(),
        name: "Flawless Victory".to_string(),
        description: "Defeat the Wave 5 boss without taking damage".to_string(),
        quest_type: QuestType::Wave,
        objectives: vec![
            QuestObjective::DefeatBossInTime("goblin_king".to_string(), 60.0),
            QuestObjective::CompleteWithoutDamage,
        ],
        rewards: QuestRewards {
            experience: 500,
            currency: vec![(CurrencyType::Gems, 10)],
            items: vec!["legendary_chest".to_string()],
            unlock: Some("special_ability".to_string()),
        },
        time_limit: None,
        repeatable: false,
    };
    
    quest_manager.available_quests.insert(wave_5_challenge.id.clone(), wave_5_challenge.clone());
    quest_manager.wave_challenges.entry(5).or_insert(Vec::new()).push(wave_5_challenge.id);
}

fn update_quest_progress(
    mut active_quests: ResMut<ActiveQuests>,
    game_stats: Res<crate::core::state::GameStats>,
    wave_manager: Res<crate::game::spawning::WaveManager>,
    combo_tracker: Res<crate::systems::combo::ComboTracker>,
) {
    // Update progress for all active quests - process each list separately to avoid borrow issues
    
    // Daily quests
    for active_quest in active_quests.daily_quests.iter_mut() {
        if active_quest.completed {
            continue;
        }
        
        for (idx, objective) in active_quest.quest.objectives.iter().enumerate() {
            let current_progress = match objective {
                QuestObjective::KillEnemies(_, target) => {
                    game_stats.enemies_killed.min(*target)
                }
                QuestObjective::ReachWave(target) => {
                    (wave_manager.current_wave).min(*target)
                }
                QuestObjective::ReachCombo(target) => {
                    combo_tracker.max_combo.min(*target)
                }
                QuestObjective::CollectItems(item_type, target) => {
                    if item_type == "coin" {
                        game_stats.coins_collected.min(*target)
                    } else {
                        active_quest.progress.get(&idx).copied().unwrap_or(0)
                    }
                }
                _ => active_quest.progress.get(&idx).copied().unwrap_or(0),
            };
            
            active_quest.progress.insert(idx, current_progress);
        }
    }
    
    // Wave challenges
    for active_quest in active_quests.wave_challenges.iter_mut() {
        if active_quest.completed {
            continue;
        }
        
        for (idx, objective) in active_quest.quest.objectives.iter().enumerate() {
            let current_progress = match objective {
                QuestObjective::KillEnemies(_, target) => {
                    game_stats.enemies_killed.min(*target)
                }
                QuestObjective::ReachWave(target) => {
                    (wave_manager.current_wave).min(*target)
                }
                QuestObjective::ReachCombo(target) => {
                    combo_tracker.max_combo.min(*target)
                }
                QuestObjective::CollectItems(item_type, target) => {
                    if item_type == "coin" {
                        game_stats.coins_collected.min(*target)
                    } else {
                        active_quest.progress.get(&idx).copied().unwrap_or(0)
                    }
                }
                _ => active_quest.progress.get(&idx).copied().unwrap_or(0),
            };
            
            active_quest.progress.insert(idx, current_progress);
        }
    }
    
    // Story quests
    for active_quest in active_quests.story_quests.iter_mut() {
        if active_quest.completed {
            continue;
        }
        
        for (idx, objective) in active_quest.quest.objectives.iter().enumerate() {
            let current_progress = match objective {
                QuestObjective::KillEnemies(_, target) => {
                    game_stats.enemies_killed.min(*target)
                }
                QuestObjective::ReachWave(target) => {
                    (wave_manager.current_wave).min(*target)
                }
                QuestObjective::ReachCombo(target) => {
                    combo_tracker.max_combo.min(*target)
                }
                QuestObjective::CollectItems(item_type, target) => {
                    if item_type == "coin" {
                        game_stats.coins_collected.min(*target)
                    } else {
                        active_quest.progress.get(&idx).copied().unwrap_or(0)
                    }
                }
                _ => active_quest.progress.get(&idx).copied().unwrap_or(0),
            };
            
            active_quest.progress.insert(idx, current_progress);
        }
    }
}

fn check_quest_completion(
    mut active_quests: ResMut<ActiveQuests>,
    mut complete_events: EventWriter<QuestCompleteEvent>,
    player_q: Query<Entity, With<crate::game::player::Player>>,
) {
    let Ok(player_entity) = player_q.single() else { return };
    
    // Process each quest list separately to avoid borrow issues
    
    // Daily quests
    for active_quest in active_quests.daily_quests.iter_mut() {
        if active_quest.completed {
            continue;
        }
        
        let all_complete = active_quest.quest.objectives.iter().enumerate().all(|(idx, obj)| {
            let progress = active_quest.progress.get(&idx).copied().unwrap_or(0);
            match obj {
                QuestObjective::KillEnemies(_, target) => progress >= *target,
                QuestObjective::CollectItems(_, target) => progress >= *target,
                QuestObjective::ReachWave(target) => progress >= *target,
                QuestObjective::ReachCombo(target) => progress >= *target,
                QuestObjective::Custom(_, target) => progress >= *target,
                _ => false, // Handle other types
            }
        });
        
        if all_complete {
            active_quest.completed = true;
            complete_events.write(QuestCompleteEvent {
                quest_id: active_quest.quest.id.clone(),
                player: player_entity,
            });
        }
    }
    
    // Wave challenges
    for active_quest in active_quests.wave_challenges.iter_mut() {
        if active_quest.completed {
            continue;
        }
        
        let all_complete = active_quest.quest.objectives.iter().enumerate().all(|(idx, obj)| {
            let progress = active_quest.progress.get(&idx).copied().unwrap_or(0);
            match obj {
                QuestObjective::KillEnemies(_, target) => progress >= *target,
                QuestObjective::CollectItems(_, target) => progress >= *target,
                QuestObjective::ReachWave(target) => progress >= *target,
                QuestObjective::ReachCombo(target) => progress >= *target,
                QuestObjective::Custom(_, target) => progress >= *target,
                _ => false, // Handle other types
            }
        });
        
        if all_complete {
            active_quest.completed = true;
            complete_events.write(QuestCompleteEvent {
                quest_id: active_quest.quest.id.clone(),
                player: player_entity,
            });
        }
    }
    
    // Story quests
    for active_quest in active_quests.story_quests.iter_mut() {
        if active_quest.completed {
            continue;
        }
        
        let all_complete = active_quest.quest.objectives.iter().enumerate().all(|(idx, obj)| {
            let progress = active_quest.progress.get(&idx).copied().unwrap_or(0);
            match obj {
                QuestObjective::KillEnemies(_, target) => progress >= *target,
                QuestObjective::CollectItems(_, target) => progress >= *target,
                QuestObjective::ReachWave(target) => progress >= *target,
                QuestObjective::ReachCombo(target) => progress >= *target,
                QuestObjective::Custom(_, target) => progress >= *target,
                _ => false, // Handle other types
            }
        });
        
        if all_complete {
            active_quest.completed = true;
            complete_events.write(QuestCompleteEvent {
                quest_id: active_quest.quest.id.clone(),
                player: player_entity,
            });
        }
    }
}

fn generate_daily_quests(
    mut active_quests: ResMut<ActiveQuests>,
    quest_manager: Res<QuestManager>,
    time: Res<Time>,
) {
    // Generate new daily quests at the start of each day
    // This would be tied to real time or game sessions
}

fn handle_quest_rewards(
    mut events: EventReader<QuestCompleteEvent>,
    quest_manager: Res<QuestManager>,
    mut currency: ResMut<crate::systems::shop::PlayerCurrency>,
    mut active_quests: ResMut<ActiveQuests>,
) {
    for event in events.read() {
        if let Some(quest) = quest_manager.available_quests.get(&event.quest_id) {
            // Apply rewards
            for (currency_type, amount) in &quest.rewards.currency {
                match currency_type {
                    CurrencyType::Coins => currency.coins += amount,
                    CurrencyType::Gems => currency.gems += amount,
                    CurrencyType::SoulShards => currency.soul_shards += amount,
                }
            }
            
            // Track completion
            *active_quests.completed_quests.entry(event.quest_id.clone()).or_insert(0) += 1;
            
            println!("Quest Complete: {}", quest.name);
        }
    }
}
