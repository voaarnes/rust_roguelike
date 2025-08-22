use bevy::prelude::*;
use crate::core::events::DamageType;

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub damage_type: DamageType,
    pub speed: f32,
    pub direction: Vec2,
    pub lifetime: Timer,
    pub piercing: u32,
    pub owner: Entity,
}

pub fn update_projectiles(
    mut projectile_q: Query<(Entity, &mut Transform, &mut Projectile)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut projectile) in projectile_q.iter_mut() {
        projectile.lifetime.tick(time.delta());
        
        // Move projectile
        transform.translation += projectile.direction.extend(0.0) * projectile.speed * time.delta_secs();
        
        // Remove if lifetime expired
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    direction: Vec2,
    damage: i32,
    damage_type: DamageType,
    owner: Entity,
) {
    commands.spawn((
        Sprite {
            color: match damage_type {
                DamageType::Fire => Color::linear_rgb(1.0, 0.5, 0.0),
                DamageType::Ice => Color::linear_rgb(0.0, 0.5, 1.0),
                DamageType::Magic => Color::linear_rgb(0.5, 0.0, 1.0),
                _ => Color::WHITE,
            },
            custom_size: Some(Vec2::new(8.0, 8.0)),
            ..default()
        },
        Transform::from_translation(position),
        Projectile {
            damage,
            damage_type,
            speed: 300.0,
            direction: direction.normalize(),
            lifetime: Timer::from_seconds(3.0, TimerMode::Once),
            piercing: 0,
            owner,
        },
        crate::game::movement::Collider { size: Vec2::new(8.0, 8.0) },
    ));
}
