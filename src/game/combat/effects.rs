use bevy::prelude::*;

#[derive(Component)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: Timer,
    pub tick_timer: Timer,
    pub stacks: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatusEffectType {
    Poison { damage_per_tick: i32 },
    Burn { damage_per_tick: i32 },
    Freeze { slow_percentage: f32 },
    Stun,
    Regeneration { heal_per_tick: i32 },
    Shield { amount: i32 },
    SpeedBoost { multiplier: f32 },
    DamageBoost { multiplier: f32 },
}

pub fn update_status_effects(
    mut query: Query<(Entity, &mut crate::game::combat::Health, &mut StatusEffect)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut health, mut effect) in query.iter_mut() {
        effect.duration.tick(time.delta());
        effect.tick_timer.tick(time.delta());
        
        if effect.tick_timer.just_finished() {
            match effect.effect_type {
                StatusEffectType::Poison { damage_per_tick } => {
                    health.take_damage(damage_per_tick * effect.stacks as i32);
                }
                StatusEffectType::Burn { damage_per_tick } => {
                    health.take_damage(damage_per_tick * effect.stacks as i32);
                }
                StatusEffectType::Regeneration { heal_per_tick } => {
                    health.heal(heal_per_tick * effect.stacks as i32);
                }
                _ => {}
            }
            effect.tick_timer.reset();
        }
        
        if effect.duration.finished() {
            commands.entity(entity).remove::<StatusEffect>();
        }
    }
}
