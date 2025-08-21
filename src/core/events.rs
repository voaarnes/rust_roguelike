use bevy::prelude::*;

#[derive(Event)]
pub enum GameEvent {
    LevelCompleted { level: usize },
    BossDefeated { boss_type: String },
    AchievementUnlocked { achievement_id: String },
    QuestCompleted { quest_id: String },
}

#[derive(Event)]
pub enum PlayerEvent {
    LevelUp { new_level: u32 },
    SkillUnlocked { skill_id: String },
    ItemPickup { item_id: String, quantity: u32 },
    Death,
}

#[derive(Event)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: i32,
    pub damage_type: DamageType,
    pub position: Vec3,
}

#[derive(Clone, Copy)]
pub enum DamageType {
    Physical,
    Magic,
    Fire,
    Ice,
    Poison,
    True, // Ignores armor
}
