use bevy::prelude::*;
use crate::game::enemy::Enemy;
use crate::game::combat::Health;
use crate::game::movement::{Velocity, Collider};
use super::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_projectiles,
                update_projectiles,
                handle_projectile_collisions,
                cleanup_expired_projectiles,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub pierce_remaining: u32,
    pub lifetime: Timer,
    pub owner: Entity,
    pub hit_entities: Vec<Entity>,
}

#[derive(Component)]
pub struct HomingProjectile {
    pub turn_speed: f32,
}

fn spawn_projectiles(
    mut commands: Commands,
    mut events: EventReader<TriggerAbilityEvent>,
    registry: Res<AbilityRegistry>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        let Some(definition) = registry.abilities.get(&event.ability_id) else { continue };

        if let AbilityType::Projectile(ref config) = definition.ability_type {
            match config.targeting {
                TargetingType::Nearest => {
                    // Find nearest enemy
                    let mut nearest_enemy = None;
                    let mut nearest_distance = f32::MAX;

                    for enemy_transform in enemy_query.iter() {
                        let distance = event.position.distance(enemy_transform.translation);
                        if distance < nearest_distance {
                            nearest_distance = distance;
                            nearest_enemy = Some(enemy_transform.translation);
                        }
                    }

                    if let Some(target_pos) = nearest_enemy {
                        let direction3 = (target_pos - event.position).normalize_or_zero();
                        spawn_single_projectile(
                            &mut commands,
                            &asset_server,
                            event.position,
                            direction3.truncate(),
                            config,
                            event.caster,
                        );
                    }
                }
                TargetingType::AllDirections => {
                    // Spawn 8 projectiles in all directions
                    for i in 0..8 {
                        let angle = (i as f32) * std::f32::consts::TAU / 8.0;
                        let direction = Vec2::new(angle.cos(), angle.sin());
                        spawn_single_projectile(
                            &mut commands,
                            &asset_server,
                            event.position,
                            direction,
                            config,
                            event.caster,
                        );
                    }
                }
                TargetingType::Forward => {
                    // For now, shoot towards mouse or default right
                    let direction = Vec2::X; // Default direction
                    spawn_single_projectile(
                        &mut commands,
                        &asset_server,
                        event.position,
                        direction,
                        config,
                        event.caster,
                    );
                }
                TargetingType::Spiral => {
                    // Spawn projectiles in a spiral pattern
                    for i in 0..3 {
                        let angle = (i as f32) * std::f32::consts::TAU / 3.0;
                        let direction = Vec2::new(angle.cos(), angle.sin());
                        spawn_single_projectile(
                            &mut commands,
                            &asset_server,
                            event.position,
                            direction,
                            config,
                            event.caster,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

fn spawn_single_projectile(
    commands: &mut Commands,
    _asset_server: &AssetServer, // kept for future use; underscore avoids warning
    position: Vec3,
    direction: Vec2,
    config: &ProjectileConfig,
    owner: Entity,
) {
    let color = match config.projectile_visual {
        ProjectileVisual::Strawberry => Color::srgb(1.0, 0.2, 0.2),
        ProjectileVisual::Pear => Color::srgb(0.7, 1.0, 0.3),
        ProjectileVisual::Mango => Color::srgb(1.0, 0.6, 0.0),
        ProjectileVisual::Pineapple => Color::srgb(1.0, 1.0, 0.0),
        ProjectileVisual::Apple => Color::srgb(0.8, 0.2, 0.2),
        ProjectileVisual::Carrot => Color::srgb(1.0, 0.5, 0.0),
        ProjectileVisual::Coconut => Color::srgb(0.6, 0.4, 0.2),
        ProjectileVisual::Energy => Color::srgb(0.0, 0.8, 1.0),
    };

    let vel = direction.normalize_or_zero() * config.speed;

    commands.spawn((
        Projectile {
            damage: config.damage,
            pierce_remaining: config.pierce_count,
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            owner,
            hit_entities: Vec::new(),
        },
        Velocity(vel),
        Collider { size: Vec2::splat(10.0) },
        Sprite {
            color,
            custom_size: Some(Vec2::splat(12.0)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 0.0, 5.0)),
    ));
}

fn update_projectiles(
    mut projectile_q: Query<(&mut Transform, &Velocity, &mut Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, velocity, mut projectile) in projectile_q.iter_mut() {
        projectile.lifetime.tick(time.delta());
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn handle_projectile_collisions(
    mut commands: Commands,
    mut projectile_q: Query<(Entity, &Transform, &mut Projectile, &Collider)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Health, &Collider), With<Enemy>>,
) {
    for (proj_entity, proj_tf, mut projectile, proj_collider) in projectile_q.iter_mut() {
        for (enemy_entity, enemy_tf, mut enemy_health, enemy_collider) in enemy_q.iter_mut() {
            // Skip if already hit this enemy
            if projectile.hit_entities.contains(&enemy_entity) {
                continue;
            }

            let distance = proj_tf.translation.distance(enemy_tf.translation);
            let collision_dist = (proj_collider.size.x + enemy_collider.size.x) / 2.0;

            if distance <= collision_dist {
                enemy_health.take_damage(projectile.damage);
                projectile.hit_entities.push(enemy_entity);

                if projectile.pierce_remaining > 0 {
                    projectile.pierce_remaining -= 1;
                } else {
                    commands.entity(proj_entity).despawn();
                    break;
                }
            }
        }
    }
}

fn cleanup_expired_projectiles(
    mut commands: Commands,
    projectile_q: Query<(Entity, &Projectile)>,
) {
    for (entity, projectile) in projectile_q.iter() {
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
