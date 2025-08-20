use bevy::prelude::*;
use std::collections::VecDeque;

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollectPowerUp>()
            .add_event::<PowerUpChanged>()
            .init_resource::<PowerUpSlots>()
            .add_systems(Update, (
                handle_powerup_collection,
                apply_powerup_effects,
                update_player_visuals,
            ));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PowerUpType {
    Strawberry,
    Pear,
    Mango,
    Apple,
    Orange,
    Grape,
    Banana,
    Cherry,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SlotPosition {
    Head,
    Chest,
    Legs,
}

#[derive(Component, Clone)]
pub struct PowerUp {
    pub power_type: PowerUpType,
    pub slot: SlotPosition,
    pub sprite_index: usize,
}

#[derive(Resource)]
pub struct PowerUpSlots {
    pub slots: VecDeque<PowerUpType>,
    pub max_slots: usize,
}

impl Default for PowerUpSlots {
    fn default() -> Self {
        Self {
            slots: VecDeque::with_capacity(3),
            max_slots: 3,
        }
    }
}

#[derive(Event)]
pub struct CollectPowerUp {
    pub power_type: PowerUpType,
}

#[derive(Event)]
pub struct PowerUpChanged {
    pub slot: SlotPosition,
    pub old_power: Option<PowerUpType>,
    pub new_power: Option<PowerUpType>,
}

impl PowerUpType {
    pub fn get_head_effect(&self) -> PlayerEffect {
        match self {
            PowerUpType::Strawberry => PlayerEffect::SpeedBoost(1.2),
            PowerUpType::Pear => PlayerEffect::VisionBoost(1.5),
            PowerUpType::Mango => PlayerEffect::CriticalChance(0.15),
            PowerUpType::Apple => PlayerEffect::HealthRegen(2.0),
            PowerUpType::Orange => PlayerEffect::JumpBoost(1.3),
            PowerUpType::Grape => PlayerEffect::DodgeChance(0.1),
            PowerUpType::Banana => PlayerEffect::AttackSpeed(1.25),
            PowerUpType::Cherry => PlayerEffect::LifeSteal(0.1),
        }
    }

    pub fn get_chest_effect(&self) -> PlayerEffect {
        match self {
            PowerUpType::Strawberry => PlayerEffect::Defense(5),
            PowerUpType::Pear => PlayerEffect::Thorns(3),
            PowerUpType::Mango => PlayerEffect::Shield(20),
            PowerUpType::Apple => PlayerEffect::MaxHealth(25),
            PowerUpType::Orange => PlayerEffect::Knockback(1.5),
            PowerUpType::Grape => PlayerEffect::Reflect(0.2),
            PowerUpType::Banana => PlayerEffect::Armor(3),
            PowerUpType::Cherry => PlayerEffect::Invulnerability(0.5),
        }
    }

    pub fn get_legs_effect(&self) -> PlayerEffect {
        match self {
            PowerUpType::Strawberry => PlayerEffect::MovementSpeed(1.4),
            PowerUpType::Pear => PlayerEffect::DashCooldown(0.7),
            PowerUpType::Mango => PlayerEffect::DoubleJump,
            PowerUpType::Apple => PlayerEffect::StaminaRegen(1.5),
            PowerUpType::Orange => PlayerEffect::SprintDuration(2.0),
            PowerUpType::Grape => PlayerEffect::SlowResist(0.5),
            PowerUpType::Banana => PlayerEffect::Slide,
            PowerUpType::Cherry => PlayerEffect::TrailDamage(2),
        }
    }

    pub fn get_sprite_indices(&self) -> (usize, usize, usize) {
        match self {
            PowerUpType::Strawberry => (0, 16, 32),
            PowerUpType::Pear => (1, 17, 33),
            PowerUpType::Mango => (2, 18, 34),
            PowerUpType::Apple => (3, 19, 35),
            PowerUpType::Orange => (4, 20, 36),
            PowerUpType::Grape => (5, 21, 37),
            PowerUpType::Banana => (6, 22, 38),
            PowerUpType::Cherry => (7, 23, 39),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PlayerEffect {
    SpeedBoost(f32),
    VisionBoost(f32),
    CriticalChance(f32),
    HealthRegen(f32),
    JumpBoost(f32),
    DodgeChance(f32),
    AttackSpeed(f32),
    LifeSteal(f32),
    Defense(i32),
    Thorns(i32),
    Shield(i32),
    MaxHealth(i32),
    Knockback(f32),
    Reflect(f32),
    Armor(i32),
    Invulnerability(f32),
    MovementSpeed(f32),
    DashCooldown(f32),
    DoubleJump,
    StaminaRegen(f32),
    SprintDuration(f32),
    SlowResist(f32),
    Slide,
    TrailDamage(i32),
}

fn handle_powerup_collection(
    mut powerup_slots: ResMut<PowerUpSlots>,
    mut collect_events: EventReader<CollectPowerUp>,
    mut change_events: EventWriter<PowerUpChanged>,
) {
    for event in collect_events.read() {
        let mut old_powers = [None, None, None];
        
        for (i, power) in powerup_slots.slots.iter().enumerate() {
            old_powers[i] = Some(*power);
        }
        
        if powerup_slots.slots.len() >= powerup_slots.max_slots {
            powerup_slots.slots.pop_front();
        }
        
        powerup_slots.slots.push_back(event.power_type);
        
        let new_powers: Vec<Option<PowerUpType>> = powerup_slots.slots
            .iter()
            .map(|p| Some(*p))
            .chain(std::iter::repeat(None))
            .take(3)
            .collect();
        
        if old_powers[0] != new_powers[0] {
            change_events.send(PowerUpChanged {
                slot: SlotPosition::Head,
                old_power: old_powers[0],
                new_power: new_powers[0],
            });
        }
        if old_powers[1] != new_powers[1] {
            change_events.send(PowerUpChanged {
                slot: SlotPosition::Chest,
                old_power: old_powers[1],
                new_power: new_powers[1],
            });
        }
        if old_powers[2] != new_powers[2] {
            change_events.send(PowerUpChanged {
                slot: SlotPosition::Legs,
                old_power: old_powers[2],
                new_power: new_powers[2],
            });
        }
    }
}

fn apply_powerup_effects(
    powerup_slots: Res<PowerUpSlots>,
    mut player_query: Query<&mut crate::entities::player::PlayerStats>,
) {
    for mut stats in player_query.iter_mut() {
        stats.reset_to_base();
        
        for (index, power_type) in powerup_slots.slots.iter().enumerate() {
            let effect = match index {
                0 => power_type.get_head_effect(),
                1 => power_type.get_chest_effect(),
                2 => power_type.get_legs_effect(),
                _ => continue,
            };
            
            stats.apply_effect(effect);
        }
    }
}

fn update_player_visuals(
    mut change_events: EventReader<PowerUpChanged>,
    mut player_query: Query<&mut crate::entities::player::PlayerVisuals>,
) {
    for event in change_events.read() {
        for mut visuals in player_query.iter_mut() {
            if let Some(power) = event.new_power {
                let indices = power.get_sprite_indices();
                match event.slot {
                    SlotPosition::Head => visuals.head_index = indices.0,
                    SlotPosition::Chest => visuals.chest_index = indices.1,
                    SlotPosition::Legs => visuals.legs_index = indices.2,
                }
            } else {
                match event.slot {
                    SlotPosition::Head => visuals.head_index = 0,
                    SlotPosition::Chest => visuals.chest_index = 16,
                    SlotPosition::Legs => visuals.legs_index = 32,
                }
            }
        }
    }
}
