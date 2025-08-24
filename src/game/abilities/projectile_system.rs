use bevy::prelude::*;
use crate::game::enemy::Enemy;
use crate::game::combat::Health;
use crate::game::movement::{Velocity, Collider};
use super::*;
use super::ability_visuals::{AnimatedSprite, ProjectileVisualEffect, FruitVisualAssets, TrailEffect};

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
                update_projectile_rotation,
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
    assets: Res<FruitVisualAssets>,
) {
    for event in events.read() {
        let Some(definition) = registry.abilities.get(&event.ability_id) else { continue };

        if let AbilityType::Projectile(ref config) = definition.ability_type {
            match config.targeting {
                TargetingType::Nearest => {
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
                        spawn_enhanced_projectile(
                            &mut commands,
                            &assets,
                            event.position,
                            direction3.truncate(),
                            config,
                            event.caster,
                            event.ability_id.fruit_type,
                        );
                    }
                }
                TargetingType::AllDirections => {
                    for i in 0..8 {
                        let angle = (i as f32) * std::f32::consts::TAU / 8.0;
                        let direction = Vec2::new(angle.cos(), angle.sin());
                        spawn_enhanced_projectile(
                            &mut commands,
                            &assets,
                            event.position,
                            direction,
                            config,
                            event.caster,
                            event.ability_id.fruit_type,
                        );
                    }
                }
                TargetingType::Forward => {
                    let direction = Vec2::X;
                    spawn_enhanced_projectile(
                        &mut commands,
                        &assets,
                        event.position,
                        direction,
                        config,
                        event.caster,
                        event.ability_id.fruit_type,
                    );
                }
                TargetingType::Spiral => {
                    for i in 0..3 {
                        let angle = (i as f32) * std::f32::consts::TAU / 3.0;
                        let direction = Vec2::new(angle.cos(), angle.sin());
                        spawn_enhanced_projectile(
                            &mut commands,
                            &assets,
                            event.position,
                            direction,
                            config,
                            event.caster,
                            event.ability_id.fruit_type,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

fn spawn_enhanced_projectile(
    commands: &mut Commands,
    assets: &FruitVisualAssets,
    position: Vec3,
    direction: Vec2,
    config: &ProjectileConfig,
    owner: Entity,
    fruit_type: u8,
) {
    let vel = direction.normalize_or_zero() * config.speed;
    
    // Calculate sprite index based on projectile visual type
    let sprite_row = match config.projectile_visual {
        ProjectileVisual::Strawberry => 0,
        ProjectileVisual::Pear => 1,
        ProjectileVisual::Mango => 2,
        ProjectileVisual::Pineapple => 3,
        ProjectileVisual::Apple => 4,
        ProjectileVisual::Carrot => 5,
        ProjectileVisual::Coconut => 6,
        ProjectileVisual::Energy => 7,
    };
    
    let first_frame = sprite_row * 4; // 4 frames per projectile type
    
    // Get rotation speed based on fruit type
    let rotation_speed = match fruit_type {
        0 => 5.0,  // Strawberry - fast spin
        1 => 2.0,  // Pear - gentle wobble
        2 => 3.0,  // Mango - medium spin
        3 => 4.0,  // Pineapple - quick spin
        4 => 1.5,  // Apple - tumble
        5 => 8.0,  // Carrot - drill
        6 => 1.0,  // Coconut - slow roll
        _ => 2.0,
    };
    
    // Base size varies by fruit
    let base_size = match fruit_type {
        6 => Vec2::splat(24.0), // Coconut is bigger
        5 => Vec2::new(20.0, 8.0), // Carrot is elongated
        _ => Vec2::splat(16.0),
    };

    let projectile_entity = commands.spawn((
        Projectile {
            damage: config.damage,
            pierce_remaining: config.pierce_count,
            lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            owner,
            hit_entities: Vec::new(),
        },
        Velocity(vel),
        Collider { size: base_size },
        Sprite {
            image: assets.projectile_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.projectile_atlas.clone(),
                index: first_frame,
            }),
            custom_size: Some(base_size * 1.5),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 0.0, 5.0)),
        AnimatedSprite {
            first_frame,
            frame_count: 4,
            frame_timer: Timer::from_seconds(0.067, TimerMode::Repeating), // 15 FPS
            current_frame: 0,
            looping: true,
        },
        ProjectileVisualEffect {
            fruit_type: config.projectile_visual.clone(),
            glow_intensity: 0.5,
            rotation_speed,
        },
    )).id();
    
    // Add trail effect for certain fruits
    if matches!(fruit_type, 0 | 1 | 2 | 3) {
        commands.entity(projectile_entity).insert(TrailEffect {
            positions: Vec::new(),
            max_length: 10,
            fade_time: 0.3,
            fruit_type,
            segment_entities: Vec::new(),
        });
    }
    
    // Add homing for certain abilities
    if fruit_type == 4 { // Apple has slight homing
        commands.entity(projectile_entity).insert(HomingProjectile {
            turn_speed: 2.0,
        });
    }
}

fn update_projectiles(
    mut projectile_q: Query<(&mut Transform, &mut Velocity, &mut Projectile, Option<&HomingProjectile>)>,
    enemy_q: Query<&Transform, (With<Enemy>, Without<Projectile>)>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, mut projectile, homing) in projectile_q.iter_mut() {
        projectile.lifetime.tick(time.delta());
        
        // Handle homing
        if let Some(homing) = homing {
            if let Some(nearest_enemy) = find_nearest_enemy(&transform.translation, &enemy_q) {
                let desired_direction = (nearest_enemy - transform.translation).normalize_or_zero();
                let current_direction = velocity.0.normalize_or_zero();
                let new_direction = current_direction.lerp(
                    desired_direction.truncate(), 
                    homing.turn_speed * time.delta_secs()
                );
                velocity.0 = new_direction * velocity.0.length();
            }
        }
        
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}

fn find_nearest_enemy(
    position: &Vec3,
    enemy_q: &Query<&Transform, (With<Enemy>, Without<Projectile>)>,
) -> Option<Vec3> {
    let mut nearest_pos = None;
    let mut nearest_dist = f32::MAX;
    
    for enemy_transform in enemy_q.iter() {
        let dist = position.distance(enemy_transform.translation);
        if dist < nearest_dist {
            nearest_dist = dist;
            nearest_pos = Some(enemy_transform.translation);
        }
    }
    
    nearest_pos
}

fn update_projectile_rotation(
    mut projectile_q: Query<(&mut Transform, &ProjectileVisualEffect, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, visual, velocity) in projectile_q.iter_mut() {
        // Rotate based on fruit type
        match visual.fruit_type {
            ProjectileVisual::Carrot => {
                // Point in direction of travel and spin
                let angle = velocity.0.y.atan2(velocity.0.x);
                transform.rotation = Quat::from_rotation_z(angle);
                transform.rotate_z(visual.rotation_speed * time.delta_secs());
            }
            ProjectileVisual::Pear | ProjectileVisual::Apple => {
                // Tumble
                transform.rotate_z(visual.rotation_speed * time.delta_secs());
                transform.rotate_x(visual.rotation_speed * 0.5 * time.delta_secs());
            }
            _ => {
                // Simple spin
                transform.rotate_z(visual.rotation_speed * time.delta_secs());
            }
        }
    }
}

fn handle_projectile_collisions(
    mut commands: Commands,
    mut projectile_q: Query<(Entity, &Transform, &mut Projectile, &Collider)>,
    mut enemy_q: Query<(Entity, &Transform, &mut Health, &Collider), With<Enemy>>,
) {
    for (proj_entity, proj_tf, mut projectile, proj_collider) in projectile_q.iter_mut() {
        for (enemy_entity, enemy_tf, mut enemy_health, enemy_collider) in enemy_q.iter_mut() {
            if projectile.hit_entities.contains(&enemy_entity) {
                continue;
            }

            let distance = proj_tf.translation.distance(enemy_tf.translation);
            let collision_dist = (proj_collider.size.x + enemy_collider.size.x) / 2.0;

            if distance <= collision_dist {
                enemy_health.take_damage(projectile.damage);
                projectile.hit_entities.push(enemy_entity);
                
                // Spawn impact effect
                spawn_impact_effect(&mut commands, proj_tf.translation, projectile.damage);

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

fn spawn_impact_effect(commands: &mut Commands, position: Vec3, damage: i32) {
    // Impact visual scales with damage
    let scale = 1.0 + (damage as f32 / 20.0);
    
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 0.8, 0.0, 0.8),
            custom_size: Some(Vec2::splat(10.0 * scale)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 0.0, 6.0))
            .with_scale(Vec3::splat(0.1)),
        ImpactEffect {
            lifetime: Timer::from_seconds(0.3, TimerMode::Once),
        },
    ));
}

#[derive(Component)]
struct ImpactEffect {
    lifetime: Timer,
}

fn cleanup_expired_projectiles(
    mut commands: Commands,
    projectile_q: Query<(Entity, &Projectile)>,
    mut impact_q: Query<(Entity, &mut ImpactEffect, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
) {
    // Cleanup projectiles
    for (entity, projectile) in projectile_q.iter() {
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
    
    // Animate and cleanup impact effects
    for (entity, mut impact, mut transform, mut sprite) in impact_q.iter_mut() {
        impact.lifetime.tick(time.delta());
        
        if impact.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            let progress = impact.lifetime.fraction();
            transform.scale = Vec3::splat(1.0 + progress * 2.0);
            sprite.color.set_alpha((1.0 - progress) * 0.8);
        }
    }
}
