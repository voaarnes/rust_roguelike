use bevy::prelude::*;
use crate::game::enemy::Enemy;
use crate::game::combat::Health;
use crate::game::player::Player;
use super::*;

pub struct AreaEffectPlugin;

impl Plugin for AreaEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_area_effects,
            update_area_effects,
            apply_area_damage,
            cleanup_expired_areas,
        ).chain());
    }
}

#[derive(Component)]
pub struct AreaEffect {
    pub damage_per_tick: i32,
    pub radius: f32,
    pub tick_timer: Timer,
    pub lifetime: Timer,
    pub owner: Entity,
    pub effect_type: AreaEffectType,
}

#[derive(Component)]
pub struct AreaVisual;

fn spawn_area_effects(
    mut commands: Commands,
    mut events: EventReader<TriggerAbilityEvent>,
    registry: Res<AbilityRegistry>,
) {
    for event in events.read() {
        let Some(definition) = registry.abilities.get(&event.ability_id) else { continue };
        
        if let AbilityType::AreaEffect(ref config) = definition.ability_type {
            let color = match config.effect_type {
                AreaEffectType::Explosion => Color::srgba(1.0, 0.5, 0.0, 0.3),
                AreaEffectType::PoisonCloud => Color::srgba(0.0, 0.8, 0.0, 0.3),
                AreaEffectType::HealingAura => Color::srgba(0.0, 1.0, 0.5, 0.3),
                AreaEffectType::SlowField => Color::srgba(0.0, 0.5, 1.0, 0.3),
                AreaEffectType::BurnGround => Color::srgba(1.0, 0.3, 0.0, 0.3),
            };
            
            commands.spawn((
                AreaEffect {
                    damage_per_tick: config.damage,
                    radius: config.radius,
                    tick_timer: Timer::from_seconds(config.tick_rate, TimerMode::Repeating),
                    lifetime: Timer::from_seconds(config.duration, TimerMode::Once),
                    owner: event.caster,
                    effect_type: config.effect_type.clone(),
                },
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(config.radius * 2.0)),
                    ..default()
                },
                Transform::from_translation(event.position + Vec3::new(0.0, 0.0, 1.0)),
                AreaVisual,
            ));
        }
    }
}

fn update_area_effects(
    mut area_q: Query<&mut AreaEffect>,
    time: Res<Time>,
) {
    for mut area in area_q.iter_mut() {
        area.tick_timer.tick(time.delta());
        area.lifetime.tick(time.delta());
    }
}


fn apply_area_damage(
    area_q: Query<(&Transform, &AreaEffect)>,
    // Explicitly disjoint: enemies never include Player
    mut enemy_q: Query<(&Transform, &mut Health), (With<Enemy>, Without<Player>)>,
    // Explicitly disjoint: players never include Enemy
    mut player_q: Query<(&Transform, &mut Health), (With<Player>, Without<Enemy>)>,
) {
    for (area_tf, area) in area_q.iter() {
        if !area.tick_timer.just_finished() {
            continue;
        }

        // Damage enemies
        if area.damage_per_tick > 0 {
            for (enemy_tf, mut enemy_health) in enemy_q.iter_mut() {
                let distance = area_tf.translation.distance(enemy_tf.translation);
                if distance <= area.radius {
                    enemy_health.take_damage(area.damage_per_tick);
                }
            }
        }

        // Heal player (negative damage)
        if area.damage_per_tick < 0 {
            for (player_tf, mut player_health) in player_q.iter_mut() {
                let distance = area_tf.translation.distance(player_tf.translation);
                if distance <= area.radius {
                    player_health.heal((-area.damage_per_tick) as i32);
                }
            }
        }
    }
}fn cleanup_expired_areas(
    mut commands: Commands,
    area_q: Query<(Entity, &AreaEffect)>,
) {
    for (entity, area) in area_q.iter() {
        if area.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
