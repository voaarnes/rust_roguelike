use bevy::prelude::*;
use std::collections::HashMap;

pub struct TalentTreePlugin;

impl Plugin for TalentTreePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TalentTree>()
            .init_resource::<PlayerTalents>()
            .add_event::<UnlockTalentEvent>()
            .add_systems(Startup, initialize_talent_tree)
            .add_systems(Update, (
                handle_talent_unlocks,
                apply_talent_effects,
                calculate_talent_points,
            ));
    }
}

#[derive(Resource, Default)]
pub struct TalentTree {
    pub trees: HashMap<TalentTreeType, TreeData>,
}

// Alias for compatibility with UI code
pub type TalentRegistry = TalentTree;

#[derive(Clone)]
pub struct TreeData {
    pub name: String,
    pub talents: HashMap<String, Talent>,
    pub connections: HashMap<String, Vec<String>>, // talent_id -> required talents
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TalentTreeType {
    Offense,
    Defense,
    Utility,
}

#[derive(Clone)]
pub struct Talent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub max_ranks: u32,
    pub cost_per_rank: u32,
    pub position: Vec2, // Position in tree UI
    pub requirements: TalentRequirements,
    pub effects: Vec<TalentEffect>,
}

#[derive(Clone)]
pub struct TalentRequirements {
    pub level: u32,
    pub prerequisite_talents: Vec<String>,
    pub points_in_tree: u32,
}

#[derive(Clone)]
pub enum TalentEffect {
    StatIncrease(StatType, f32),
    PercentIncrease(StatType, f32),
    UnlockFeature(String),
    ModifyAbility(String, AbilityModification),
}

#[derive(Clone)]
pub enum StatType {
    Health,
    Damage,
    Speed,
    AttackSpeed,
    CritChance,
    CritDamage,
    Armor,
    CooldownReduction,
}

#[derive(Clone)]
pub enum AbilityModification {
    ExtraProjectiles(u32),
    IncreaseRadius(f32),
    AddEffect(String),
    ReduceCooldown(f32),
}

#[derive(Resource)]
pub struct PlayerTalents {
    pub unlocked_talents: HashMap<String, u32>, // talent_id -> current rank
    pub available_points: u32,
    pub spent_points: HashMap<TalentTreeType, u32>,
}

#[derive(Event)]
pub struct UnlockTalentEvent {
    pub talent_id: String,
    pub tree_type: TalentTreeType,
}

impl Default for PlayerTalents {
    fn default() -> Self {
        Self {
            unlocked_talents: HashMap::new(),
            available_points: 0,
            spent_points: HashMap::new(),
        }
    }
}

impl PlayerTalents {
    pub fn get_talent_bonus(&self, bonus_type: &str) -> Option<f32> {
        let mut total_bonus = 0.0;
        
        for (talent_id, rank) in &self.unlocked_talents {
            // This is a simplified approach - in a real game you'd look up 
            // the talent definition and calculate bonuses properly
            match bonus_type {
                "cooldown_reduction" if talent_id.contains("cooldown") => {
                    total_bonus += 0.05 * (*rank as f32); // 5% per rank
                }
                "damage_increase" if talent_id.contains("damage") || talent_id == "sharp_blade" => {
                    total_bonus += 0.1 * (*rank as f32); // 10% per rank
                }
                "health_increase" if talent_id == "thick_skin" => {
                    total_bonus += 10.0 * (*rank as f32); // 10 HP per rank
                }
                _ => {}
            }
        }
        
        if total_bonus > 0.0 {
            Some(total_bonus)
        } else {
            None
        }
    }
}

fn initialize_talent_tree(mut talent_tree: ResMut<TalentTree>) {
    // Offense Tree
    let mut offense_tree = TreeData {
        name: "Path of Destruction".to_string(),
        talents: HashMap::new(),
        connections: HashMap::new(),
    };
    
    offense_tree.talents.insert("sharp_blade".to_string(), Talent {
        id: "sharp_blade".to_string(),
        name: "Sharp Blade".to_string(),
        description: "+3% damage per rank".to_string(),
        icon: "icons/sword.png".to_string(),
        max_ranks: 5,
        cost_per_rank: 1,
        position: Vec2::new(0.0, 0.0),
        requirements: TalentRequirements {
            level: 1,
            prerequisite_talents: vec![],
            points_in_tree: 0,
        },
        effects: vec![TalentEffect::PercentIncrease(StatType::Damage, 0.03)],
    });
    
    offense_tree.talents.insert("critical_precision".to_string(), Talent {
        id: "critical_precision".to_string(),
        name: "Critical Precision".to_string(),
        description: "+2% crit chance per rank".to_string(),
        icon: "icons/crit.png".to_string(),
        max_ranks: 10,
        cost_per_rank: 1,
        position: Vec2::new(-50.0, -50.0),
        requirements: TalentRequirements {
            level: 3,
            prerequisite_talents: vec!["sharp_blade".to_string()],
            points_in_tree: 5,
        },
        effects: vec![TalentEffect::PercentIncrease(StatType::CritChance, 0.02)],
    });
    
    offense_tree.talents.insert("multishot".to_string(), Talent {
        id: "multishot".to_string(),
        name: "Multishot".to_string(),
        description: "Projectiles fire +1 additional projectile".to_string(),
        icon: "icons/multishot.png".to_string(),
        max_ranks: 3,
        cost_per_rank: 3,
        position: Vec2::new(50.0, -50.0),
        requirements: TalentRequirements {
            level: 10,
            prerequisite_talents: vec!["sharp_blade".to_string()],
            points_in_tree: 10,
        },
        effects: vec![TalentEffect::ModifyAbility(
            "projectile".to_string(),
            AbilityModification::ExtraProjectiles(1)
        )],
    });
    
    talent_tree.trees.insert(TalentTreeType::Offense, offense_tree);
    
    // Defense Tree
    let mut defense_tree = TreeData {
        name: "Path of Resilience".to_string(),
        talents: HashMap::new(),
        connections: HashMap::new(),
    };
    
    defense_tree.talents.insert("thick_skin".to_string(), Talent {
        id: "thick_skin".to_string(),
        name: "Thick Skin".to_string(),
        description: "+10 health per rank".to_string(),
        icon: "icons/shield.png".to_string(),
        max_ranks: 10,
        cost_per_rank: 1,
        position: Vec2::new(0.0, 0.0),
        requirements: TalentRequirements {
            level: 1,
            prerequisite_talents: vec![],
            points_in_tree: 0,
        },
        effects: vec![TalentEffect::StatIncrease(StatType::Health, 10.0)],
    });
    
    talent_tree.trees.insert(TalentTreeType::Defense, defense_tree);
    
    // Utility Tree
    let mut utility_tree = TreeData {
        name: "Path of Wisdom".to_string(),
        talents: HashMap::new(),
        connections: HashMap::new(),
    };
    
    utility_tree.talents.insert("swift_feet".to_string(), Talent {
        id: "swift_feet".to_string(),
        name: "Swift Feet".to_string(),
        description: "+5% movement speed per rank".to_string(),
        icon: "icons/boots.png".to_string(),
        max_ranks: 5,
        cost_per_rank: 1,
        position: Vec2::new(0.0, 0.0),
        requirements: TalentRequirements {
            level: 1,
            prerequisite_talents: vec![],
            points_in_tree: 0,
        },
        effects: vec![TalentEffect::PercentIncrease(StatType::Speed, 0.05)],
    });
    
    talent_tree.trees.insert(TalentTreeType::Utility, utility_tree);
}

fn handle_talent_unlocks(
    mut events: EventReader<UnlockTalentEvent>,
    mut player_talents: ResMut<PlayerTalents>,
    talent_tree: Res<TalentTree>,
) {
    for event in events.read() {
        if let Some(tree) = talent_tree.trees.get(&event.tree_type) {
            if let Some(talent) = tree.talents.get(&event.talent_id) {
                let current_rank = player_talents.unlocked_talents.get(&event.talent_id).copied().unwrap_or(0);
                
                if current_rank < talent.max_ranks && player_talents.available_points >= talent.cost_per_rank {
                    // Check requirements
                    let points_in_tree = player_talents.spent_points.get(&event.tree_type).copied().unwrap_or(0);
                    if points_in_tree >= talent.requirements.points_in_tree {
                        // Unlock talent
                        player_talents.unlocked_talents.insert(event.talent_id.clone(), current_rank + 1);
                        player_talents.available_points -= talent.cost_per_rank;
                        *player_talents.spent_points.entry(event.tree_type).or_insert(0) += talent.cost_per_rank;
                    }
                }
            }
        }
    }
}

fn apply_talent_effects(
    player_talents: Res<PlayerTalents>,
    talent_tree: Res<TalentTree>,
    mut player_q: Query<&mut crate::combat::CombatStats, With<crate::player::Player>>,
) {
    // Apply all unlocked talent effects to player
}

fn calculate_talent_points(
    mut player_talents: ResMut<PlayerTalents>,
    player_q: Query<&crate::player::Player>,
) {
    if let Ok(player) = player_q.single() {
        // Grant 1 talent point per level
        let expected_points = player.level;
        let spent_points: u32 = player_talents.spent_points.values().sum();
        player_talents.available_points = expected_points.saturating_sub(spent_points);
    }
}
