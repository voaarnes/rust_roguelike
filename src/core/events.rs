use bevy::prelude::*;

#[derive(Event)]
pub struct GameEvent {
    pub event_type: GameEventType,
}

#[derive(Event)]
pub struct PlayerEvent {
    pub player: Entity,
    pub event_type: PlayerEventType,
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
pub enum GameEventType {
    WaveCompleted,
    BossDefeated,
    PlayerLevelUp,
}

#[derive(Clone, Copy)]
pub enum PlayerEventType {
    LevelUp,
    PowerUpGained,
    Died,
}

#[derive(Clone, Copy)]
pub enum DamageType {
    Physical,
    Magic,
    Fire,
    Ice,
    Poison,
    True,
}
