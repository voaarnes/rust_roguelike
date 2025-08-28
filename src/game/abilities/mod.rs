pub mod ability_types;
pub mod projectile_system;
pub mod area_effects;
pub mod ability_visuals;
pub mod test_setup;

use bevy::prelude::*;
use crate::entities::powerup::PowerUpSlots;
use crate::game::player::Player;
use crate::systems::talents::PlayerTalents;
use crate::systems::shop::PlayerCurrency;
use std::collections::HashMap;

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AbilityRegistry>()
            .add_event::<TriggerAbilityEvent>()
            .add_plugins((
                projectile_system::ProjectilePlugin,
                area_effects::AreaEffectPlugin,
                ability_visuals::AbilityVisualsPlugin,
                test_setup::AbilityTestPlugin,
            ))
            .add_systems(Startup, setup_ability_registry)
            .add_systems(Update, (
                update_player_abilities,
                trigger_abilities,
                update_ability_cooldowns,
            ).chain());
    }
}

/// Component that tracks active abilities for each body part
#[derive(Component, Default)]
pub struct ActiveAbilities {
    pub head_ability: Option<AbilityInstance>,
    pub torso_ability: Option<AbilityInstance>,
    pub legs_ability: Option<AbilityInstance>,
}

/// Instance of an active ability with its cooldown
#[derive(Clone)]
pub struct AbilityInstance {
    pub ability_id: AbilityId,
    pub cooldown_timer: Timer,
    pub auto_cast: bool,
}

/// Unique identifier for each ability combination
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AbilityId {
    pub fruit_type: u8,
    pub body_part: BodyPart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BodyPart {
    Head,
    Torso,
    Legs,
}

/// Resource that stores all ability definitions
#[derive(Resource, Default)]
pub struct AbilityRegistry {
    pub abilities: HashMap<AbilityId, AbilityDefinition>,
}

/// Definition of an ability's properties
#[derive(Clone)]
pub struct AbilityDefinition {
    pub name: String,
    pub description: String,
    pub cooldown: f32,
    pub ability_type: AbilityType,
    pub visual_effect: VisualEffectType,
}

#[derive(Clone)]
pub enum AbilityType {
    Projectile(ProjectileConfig),
    AreaEffect(AreaEffectConfig),
    Buff(BuffConfig),
    Summon(SummonConfig),
}

#[derive(Clone)]
pub struct ProjectileConfig {
    pub damage: i32,
    pub speed: f32,
    pub pierce_count: u32,
    pub targeting: TargetingType,
    pub projectile_visual: ProjectileVisual,
}

#[derive(Clone)]
pub struct AreaEffectConfig {
    pub damage: i32,
    pub radius: f32,
    pub tick_rate: f32,
    pub duration: f32,
    pub effect_type: AreaEffectType,
}

#[derive(Clone)]
pub struct BuffConfig {
    pub stat_modifier: StatModifier,
    pub duration: f32,
}

#[derive(Clone)]
pub struct SummonConfig {
    pub summon_type: SummonType,
    pub duration: f32,
    pub count: u32,
}

#[derive(Clone)]
pub enum TargetingType {
    Nearest,
    Random,
    AllDirections,
    Forward,
    Spiral,
}

#[derive(Clone)]
pub enum ProjectileVisual {
    Strawberry,
    Pear,
    Mango,
    Pineapple,
    Apple,
    Carrot,
    Coconut,
    Energy,
}

#[derive(Clone)]
pub enum AreaEffectType {
    Explosion,
    PoisonCloud,
    HealingAura,
    SlowField,
    BurnGround,
}

#[derive(Clone)]
pub enum StatModifier {
    SpeedBoost(f32),
    DamageBoost(f32),
    ArmorBoost(i32),
    LifeSteal(f32),
}

#[derive(Clone)]
pub enum SummonType {
    Turret,
    Orb,
    Shield,
    Minion,
}

#[derive(Clone)]
pub enum VisualEffectType {
    None,
    Particles(ParticleConfig),
    Trail,
    Pulse,
    Aura,
}

#[derive(Clone)]
pub struct ParticleConfig {
    pub color: Color,
    pub count: u32,
    pub spread: f32,
    pub lifetime: f32,
}

#[derive(Event)]
pub struct TriggerAbilityEvent {
    pub ability_id: AbilityId,
    pub caster: Entity,
    pub position: Vec3,
}

fn setup_ability_registry(mut registry: ResMut<AbilityRegistry>) {
    use ability_types::*;
    
    // Register all 21 abilities (3 parts × 7 fruits)
    register_all_abilities(&mut registry);
}

fn update_player_abilities(
    mut player_q: Query<(&PowerUpSlots, &mut ActiveAbilities), (With<Player>, Changed<PowerUpSlots>)>,
    registry: Res<AbilityRegistry>,
) {
    for (powerup_slots, mut active_abilities) in player_q.iter_mut() {
        // Update head ability
        if let Some(fruit_type) = powerup_slots.get_head_fruit() {
            let ability_id = AbilityId {
                fruit_type,
                body_part: BodyPart::Head,
            };
            
            if let Some(definition) = registry.abilities.get(&ability_id) {
                active_abilities.head_ability = Some(AbilityInstance {
                    ability_id,
                    cooldown_timer: Timer::from_seconds(definition.cooldown, TimerMode::Repeating),
                    auto_cast: true,
                });
            }
        } else {
            active_abilities.head_ability = None;
        }
        
        // Update torso ability
        if let Some(fruit_type) = powerup_slots.get_torso_fruit() {
            let ability_id = AbilityId {
                fruit_type,
                body_part: BodyPart::Torso,
            };
            
            if let Some(definition) = registry.abilities.get(&ability_id) {
                active_abilities.torso_ability = Some(AbilityInstance {
                    ability_id,
                    cooldown_timer: Timer::from_seconds(definition.cooldown, TimerMode::Repeating),
                    auto_cast: true,
                });
            }
        } else {
            active_abilities.torso_ability = None;
        }
        
        // Update legs ability
        if let Some(fruit_type) = powerup_slots.get_legs_fruit() {
            let ability_id = AbilityId {
                fruit_type,
                body_part: BodyPart::Legs,
            };
            
            if let Some(definition) = registry.abilities.get(&ability_id) {
                active_abilities.legs_ability = Some(AbilityInstance {
                    ability_id,
                    cooldown_timer: Timer::from_seconds(definition.cooldown, TimerMode::Repeating),
                    auto_cast: true,
                });
            }
        } else {
            active_abilities.legs_ability = None;
        }
    }
}

fn trigger_abilities(
    mut player_q: Query<(Entity, &Transform, &mut ActiveAbilities), With<Player>>,
    mut trigger_events: EventWriter<TriggerAbilityEvent>,
    talents: Res<PlayerTalents>,
    time: Res<Time>,
) {
    // Calculate talent bonuses
    let cooldown_reduction = talents.get_talent_bonus("cooldown_reduction").unwrap_or(0.0);
    let damage_bonus = talents.get_talent_bonus("damage_increase").unwrap_or(0.0);
    
    for (entity, transform, mut abilities) in player_q.iter_mut() {
        // Check and trigger head ability
        if let Some(ref mut ability) = abilities.head_ability {
            // Apply cooldown reduction from talents
            let mut modified_timer = ability.cooldown_timer.clone();
            if cooldown_reduction > 0.0 {
                let reduced_duration = modified_timer.duration().as_secs_f32() * (1.0 - cooldown_reduction);
                modified_timer.set_duration(std::time::Duration::from_secs_f32(reduced_duration));
            }
            modified_timer.tick(time.delta());
            
            if modified_timer.just_finished() && ability.auto_cast {
                trigger_events.write(TriggerAbilityEvent {
                    ability_id: ability.ability_id,
                    caster: entity,
                    position: transform.translation,
                });
                ability.cooldown_timer.reset();
            }
        }
        
        // Check and trigger torso ability
        if let Some(ref mut ability) = abilities.torso_ability {
            ability.cooldown_timer.tick(time.delta());
            if ability.cooldown_timer.just_finished() && ability.auto_cast {
                trigger_events.write(TriggerAbilityEvent {
                    ability_id: ability.ability_id,
                    caster: entity,
                    position: transform.translation,
                });
            }
        }
        
        // Check and trigger legs ability
        if let Some(ref mut ability) = abilities.legs_ability {
            ability.cooldown_timer.tick(time.delta());
            if ability.cooldown_timer.just_finished() && ability.auto_cast {
                trigger_events.write(TriggerAbilityEvent {
                    ability_id: ability.ability_id,
                    caster: entity,
                    position: transform.translation,
                });
            }
        }
    }
}

fn update_ability_cooldowns(
    _time: Res<Time>,
) {
    // Additional cooldown management if needed
}

pub mod cooldown_display;

// Add this to the AbilitiesPlugin build function after the existing plugins:
// app.add_plugins(cooldown_display::CooldownDisplayPlugin);
